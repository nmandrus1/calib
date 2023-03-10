use super::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;
use uuid::Uuid;

// NOTE: Keep fields in order based on how comparisons should go,
// see Ord/PartialOrd Trait derive documentation
/// Struct to represent a given event on the calendar
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Clone)]
pub struct Event {
    start: NaiveDateTime,
    end: NaiveDateTime,
    name: String,
    id: Uuid,
}

impl Event {
    /// given a start and end time determine whether they would be valid
    fn start_end_times_valid(st: &NaiveDateTime, end: &NaiveDateTime) -> bool {
        end.signed_duration_since(*st).num_seconds().is_positive()
    }

    /// return the NaiveDate component of the start field
    pub fn start(&self) -> NaiveDateTime {
        self.start
    }

    /// return the NaiveDate component of the end field
    pub fn end(&self) -> NaiveDateTime {
        self.end
    }

    /// returns the name of the event
    pub fn name(&self) -> &str {
        &self.name
    }

    /// returns the id of the event
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Create an Event with a name and date, defaults to an
    /// all day event starting at 00:00:00 and ending at 23:59:59
    pub fn new(name: String, date: &NaiveDate) -> Self {
        Self {
            name,
            start: NaiveDateTime::new(*date, day_start()),
            end: NaiveDateTime::new(*date, day_end()),
            id: Uuid::new_v4(),
        }
    }

    /// Set/Change the date and time of the start field
    pub fn set_start(self, start: NaiveDateTime) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        if Event::start_end_times_valid(&start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event { start, ..self })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change an event's start time
    pub fn set_start_time(self, start: NaiveTime) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        let new_start = NaiveDateTime::new(self.start.date(), start);
        if Event::start_end_times_valid(&new_start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event {
                start: new_start,
                ..self
            })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change an event's start date
    pub fn set_start_date(self, start: NaiveDate) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        let new_start = NaiveDateTime::new(start, self.start.time());
        if Event::start_end_times_valid(&new_start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event {
                start: new_start,
                ..self
            })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change the date and time of the end field
    pub fn set_end(self, end: NaiveDateTime) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        if Event::start_end_times_valid(&self.start, &end) {
            // previous end time is overwritten
            Ok(Event { end, ..self })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Set/Change the time of the end field
    pub fn set_end_time(self, end: NaiveTime) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        let new_end = NaiveDateTime::new(self.end.date(), end);
        if Event::start_end_times_valid(&self.start, &new_end) {
            // previous end time is overwritten
            Ok(Event {
                end: new_end,
                ..self
            })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Set/Change the date of the end field
    pub fn set_end_date(self, end: NaiveDate) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        let new_end = NaiveDateTime::new(end, self.end.time());
        if Event::start_end_times_valid(&self.start, &new_end) {
            // previous end time is overwritten
            Ok(Event {
                end: new_end,
                ..self
            })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Change the name of an event
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
