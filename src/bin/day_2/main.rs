use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Observation {
    safe: bool,
    levels: (usize, usize),
}

impl Observation {
    fn new(first: usize, second: usize, previous_observation: Option<Observation>) -> Observation {
        let distance = first.abs_diff(second);
        let ordering = second.cmp(&first);

        let mut safe = distance <= 3 && ordering != Ordering::Equal;

        if let Some(previous_observation) = previous_observation {
            safe = safe
                && previous_observation.safe
                && previous_observation.ordering() == ordering;
        }

        Observation {
            safe,
            levels: (first, second),
        }
    }

    fn ordering(&self) -> Ordering {
        self.levels.1.cmp(&self.levels.0)
    }
}

#[derive(Copy, Clone)]
struct SafetyObserver {
    observation: Option<Observation>,
}

impl SafetyObserver {
    fn new() -> Self {
        Self { observation: None }
    }

    fn observe(self, left: usize, right: usize) -> Self {
        Self {
            observation: Some(Observation::new(left, right, self.observation)),
        }
    }

    fn is_safe(&self) -> bool {
        matches!(self.observation, None | Some(Observation { safe: true, .. }))
    }
}


#[derive(Copy, Clone)]
struct ToleranceObserver {
    safety_observer: SafetyObserver,
    candidates_for_elimination: Option<(usize, usize)>,
    already_tolerated: bool,
}

impl ToleranceObserver {
    fn new() -> Self {
        Self {
            safety_observer: SafetyObserver::new(),
            candidates_for_elimination: None,
            already_tolerated: false,
        }
    }

    fn observe(self, left: usize, right: usize) -> Self {
        if self.already_tolerated {
            return self;
        }

        let safety_observer = self.safety_observer.observe(left, right);

        if safety_observer.is_safe() {
            return Self {
                safety_observer,
                ..self
            };
        }

        match self.candidates_for_elimination {
            None => Self {
                candidates_for_elimination: Some((left, right)),
                ..self
            },

            Some(candidates) => {
                // Try to eliminate one of the candidates.
                let safe_observer = [candidates.0, candidates.1]
                    .map(|candidate| self.safety_observer.observe(candidate, right))
                    .iter().filter(|observer| observer.is_safe())
                    .next()
                    .copied();

                match safe_observer {
                    // One of the candidates was successfully eliminated.
                    Some(safety_observer) => {
                        Self {
                            safety_observer,
                            already_tolerated: true,
                            ..self
                        }
                    }
                    // No luck, already unsafe.
                    None => {
                        Self {
                            safety_observer,
                            ..self
                        }
                    }
                }
            }
        }
    }

    fn is_safe(&self) -> bool {
        self.safety_observer.is_safe() && !self.already_tolerated
    }
}

fn main() {
    let input = include_str!("input.txt");

    let reports = input
        .lines()
        .map(|report| report.split_whitespace()
            .map(|level| level.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let safe_reports = reports.iter().filter(|report| {
        let observer = report.iter().tuple_windows().fold(SafetyObserver::new(), |observer, (&left, &right)| {
            observer.observe(left, right)
        });
        observer.is_safe()
    });

    println!("Safe reports: {}", safe_reports.count());

    let tolerated_reports = reports.iter().filter(|report| {
        let observer = report.iter().tuple_windows().fold(ToleranceObserver::new(), |observer, (&left, &right)| {
            observer.observe(left, right)
        });
        observer.is_safe()
    });

    println!("Tolerated reports: {}", tolerated_reports.count());
}
