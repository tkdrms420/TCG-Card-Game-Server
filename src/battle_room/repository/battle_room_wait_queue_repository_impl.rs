use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::entity::battle_room_wait_queue::BattleRoomWaitingQueue;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;


pub struct BattleRoomWaitQueueRepositoryImpl {
    wait_queue: BattleRoomWaitingQueue,
}

impl BattleRoomWaitQueueRepositoryImpl {
    pub fn new() -> Self {
        BattleRoomWaitQueueRepositoryImpl {
            wait_queue: BattleRoomWaitingQueue::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleRoomWaitQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomWaitQueueRepository for BattleRoomWaitQueueRepositoryImpl {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        self.wait_queue.enqueue_player(account_unique_id).await;

        Ok(true)
    }

    async fn dequeue_two_players_from_wait_queue(&self, count: usize) -> Vec<i32> {
        self.wait_queue.dequeue_n_players(count).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_enqueue_player_id() {
        let repository = BattleRoomWaitQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let result = repository_arc
            .lock()
            .await
            .enqueue_player_id_for_wait(1)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enqueue_and_dequeue_players() {
        let repository = BattleRoomWaitQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let repository_guard = repository_arc.lock().await;
        repository_guard.enqueue_player_id_for_wait(1).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(2).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(3).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(4).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(5).await.expect("Can't enqueue");

        let queue_before_dequeue = repository_guard.wait_queue.player_id_list.lock().await;
        println!("Before dequeue: {:?}", queue_before_dequeue);
        drop(queue_before_dequeue);

        let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
        println!("dequeued_players: {:?}", dequeued_players);

        let queue_after_dequeue = repository_guard.wait_queue.player_id_list.lock().await;
        println!("After dequeue: {:?}", queue_after_dequeue);
        drop(queue_after_dequeue);

        let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
        println!("dequeued_players: {:?}", dequeued_players);

        let queue_after_dequeue = repository_guard.wait_queue.player_id_list.lock().await;
        println!("After dequeue: {:?}", queue_after_dequeue);
        drop(queue_after_dequeue);
    }

}
