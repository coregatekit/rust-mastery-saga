trait QuestNotifier {
    fn notify(&self, message: &str);
}

struct Pigeon;

impl QuestNotifier for Pigeon {
    fn notify(&self, message: &str) {
        println!("Pigeon says: {}", message);
    }
}

struct Email;

impl QuestNotifier for Email {
    fn notify(&self, message: &str) {
        println!("Email sent with message: {}", message);
    }
}

struct QuestManager;

impl QuestManager {
    fn complete_quest<T: QuestNotifier>(&self, notifier: T) {
        notifier.notify("Quest completed!");
    }
}

pub fn challenge() {
  let quest_manager = QuestManager;

  let pigeon_notifier = Pigeon;
  quest_manager.complete_quest(pigeon_notifier);
  let email_notifier = Email;
  quest_manager.complete_quest(email_notifier);
}
