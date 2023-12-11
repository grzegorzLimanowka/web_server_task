// // #[async_trait]
// // trait Manage {
// //     async fn
// // }

// use sea_orm::DatabaseConnection;

// use super::{Client, FetchResources};

// /// Manager responsible for performing HTTP requests and saving them to DB
// // struct Manager<T: FetchResources> {
// //     client: T,
// // }

// trait Manage<F, T>
// where
//     F: FetchResources<T>,
// {
//     //
// }
// struct Manager<F> {
//     client: F,
//     db: DatabaseConnection,
// }

// impl Manage<Client, u32> for Manager<Client> {
//     //
// }

// // impl Manage<F, T> for Manager<F> {
// // }
