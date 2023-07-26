use sqlx::Acquire;
use tokio_stream::StreamExt;

lazy_static::lazy_static! {
    static ref DBPOOL: sqlx::Pool<sqlx::Sqlite> =
        sqlx::SqlitePool::connect_lazy("sqlite:database?mode=rwc")
            .expect("should be able to create database");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut db_pool = DBPOOL.acquire().await?;
    let db = db_pool.acquire().await?;
    let _row = sqlx::query("CREATE TABLE IF NOT EXISTS entries (id INTEGER PRIMARY KEY, message TEXT NOT NULL);").execute(db).await?;

    let (entry_sender, entry_receiver) = kanal::bounded_async(5);
    let reader_worker = tokio::spawn(async move {
        read_csv_into_kanal("../todo-task/import.csv".to_string(), entry_sender).await
    });

    let mut workers = vec![];
    for _i in 0..num_cpus::get() {
        let receiver_clone = entry_receiver.clone();
        workers.push(tokio::spawn(async move {
            save_entry_to_database(receiver_clone).await
        }));
    }

    let chain = workers.into_iter().chain(std::iter::once(reader_worker));
    for res in futures::future::join_all(chain).await {
        res??;
    }
    
    print_all_entries().await?;

    Ok(())
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow)]
struct OurEntry {
    id: i64,
    message: String,
}

async fn read_csv_into_kanal(
    filename: String,
    entry_sender: kanal::AsyncSender<OurEntry>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reader = tokio::fs::File::open(filename).await?;

    let csvreader = csv_async::AsyncReaderBuilder::new()
        .delimiter(b',')
        .create_deserializer(reader);
    let mut recordstream = csvreader.into_deserialize::<OurEntry>();

    while let Some(record) = recordstream.next().await {
        let record = record?;
        entry_sender.send(record).await?;
    }
    Ok(())
}

async fn save_entry_to_database(
    entry_receiver: kanal::AsyncReceiver<OurEntry>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut db_pool = DBPOOL.acquire().await?;
    while let Ok(our_entry) = entry_receiver.recv().await {
        let db = db_pool.acquire().await?;
        let _res = sqlx::query("INSERT INTO entries (id, message) VALUES( $1, $2) ON CONFLICT(id) DO UPDATE SET message=excluded.message;")
            .bind(our_entry.id)
            .bind(our_entry.message)
            .execute(db)
            .await?;
    }
    Ok(())
}

async fn print_all_entries() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut db_pool = DBPOOL.acquire().await?;
    let db = db_pool.acquire().await?;
    let entries: Vec<OurEntry> = sqlx::query_as("Select * from entries;").fetch_all(db)
        .await?;
    for entry in entries {
        println!("{:?}",entry);
    }
    Ok(())
}
