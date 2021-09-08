use ic_cdk::export::Principal;

type TimestampMillis = u64;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
}

pub struct CanisterEnvironment {}

impl Environment for CanisterEnvironment {
    fn now(&self) -> TimestampMillis {
        ic_cdk::api::time()
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }
}

pub struct TestEnvironment {
    pub now: TimestampMillis,
    pub caller: Principal,
}

impl Environment for TestEnvironment {
    fn now(&self) -> TimestampMillis {
        self.now
    }

    fn caller(&self) -> Principal {
        self.caller
    }
}
