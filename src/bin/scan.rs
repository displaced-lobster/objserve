use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::{env, ffi::OsStr, path::Path};
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut count = 0;
    let obj_path = env::var("OBJ_PATH")?;
    let file_prefix = format!("{}/", &obj_path);

    for entry in WalkDir::new(&obj_path)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let extension = path.extension().and_then(OsStr::to_str);

        if let Some("stl") = extension {
            let src = path
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(&file_prefix, "");
            let name = src.replace('/', " - ");
            let collection = {
                if entry.depth() == 1 {
                    None
                } else {
                    Some(write_collection(&name, &src, &pool).await?)
                }
            };

            write_obj(path, &pool, collection).await?;
            count += 1;
        }
    }

    println!("{} models scanned", count);

    Ok(())
}

async fn write_collection(name: &str, src: &str, pool: &SqlitePool) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
        INSERT OR IGNORE INTO collections (name, src)
        VALUES (?1, ?2)
        "#,
        name,
        src,
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn write_obj(path: &Path, pool: &SqlitePool, collection: Option<i64>) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let name = path.file_name().unwrap().to_str().unwrap();
    let src = path.to_str().unwrap().to_string().replace("./", "");

    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO objs (name, src, collection)
        VALUES (?1, ?2, ?3)
        "#,
        name,
        src,
        collection,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
