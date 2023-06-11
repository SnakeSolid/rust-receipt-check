mod data;

pub use self::data::Reply;
pub use self::data::ReplyItem;
pub use self::data::UpdateParams;

use crate::database::Database;
use std::convert::Infallible;

macro_rules! no_fail {
    ($message:expr, $callback:expr) => {
        match $callback {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                let message = format!("{}", error);

                return Ok(warp::reply::json(&Reply::error(&message)));
            }
        }
    };
}

pub async fn list(database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("Request categories");

    let items = no_fail!(
        "Failed to read items",
        database.select_category_names().await
    );

    Ok(warp::reply::json(&Reply::list(items)))
}

pub async fn update(
    params: UpdateParams,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Request category update: {:?}", params);

     no_fail!(
        "Failed to save category",
        database
            .updatre_product_category(params.product(), params.category(), params.name())
            .await
    );

    Ok(warp::reply::json(&Reply::success()))
}
