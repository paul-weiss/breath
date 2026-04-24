#[derive(Clone, Copy, PartialEq)]
pub enum PhaseKind {
    Inhale,
    HoldFull,
    Exhale,
    HoldEmpty,
    Interactive,
}

#[derive(Clone)]
pub struct Phase {
    pub label: &'static str,
    pub kind: PhaseKind,
    pub duration_secs: f32, // 0.0 = interactive, waits for Enter
    pub hint: Option<&'static str>,
}

impl Phase {
    pub fn inhale(secs: f32) -> Self {
        Self { label: "INHALE", kind: PhaseKind::Inhale, duration_secs: secs, hint: None }
    }
    pub fn inhale_with(secs: f32, hint: &'static str) -> Self {
        Self { label: "INHALE", kind: PhaseKind::Inhale, duration_secs: secs, hint: Some(hint) }
    }
    pub fn exhale(secs: f32) -> Self {
        Self { label: "EXHALE", kind: PhaseKind::Exhale, duration_secs: secs, hint: None }
    }
    pub fn exhale_with(secs: f32, hint: &'static str) -> Self {
        Self { label: "EXHALE", kind: PhaseKind::Exhale, duration_secs: secs, hint: Some(hint) }
    }
    pub fn hold_full(secs: f32) -> Self {
        Self { label: "HOLD", kind: PhaseKind::HoldFull, duration_secs: secs, hint: Some("lungs full") }
    }
    pub fn hold_empty(secs: f32) -> Self {
        Self { label: "HOLD", kind: PhaseKind::HoldEmpty, duration_secs: secs, hint: Some("lungs empty") }
    }
    pub fn interactive(label: &'static str, hint: &'static str) -> Self {
        Self { label, kind: PhaseKind::Interactive, duration_secs: 0.0, hint: Some(hint) }
    }
}

#[derive(Clone, Copy)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
        }
    }
}

pub struct Exercise {
    pub name: &'static str,
    pub purpose: &'static str,
    pub difficulty: Difficulty,
    pub rounds: u32,
    pub phases: Vec<Phase>,
    pub intro: &'static str,
    pub completion_note: Option<&'static str>,
    pub warning: Option<&'static str>,
}

pub fn all() -> Vec<Exercise> {
    vec![
        Exercise {
            name: "Coherent Breathing",
            purpose: "Balance the nervous system — improve HRV and reduce stress",
            difficulty: Difficulty::Beginner,
            rounds: 20,
            phases: vec![
                Phase::inhale(5.5),
                Phase::exhale(5.5),
            ],
            intro: "Breathe at 5.5 breaths per minute.\nIn through the nose for 5.5 seconds, out through the nose for 5.5 seconds.\nLet the rhythm settle. No forcing.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Box Breathing",
            purpose: "Focus under stress — used by military and athletes",
            difficulty: Difficulty::Beginner,
            rounds: 10,
            phases: vec![
                Phase::inhale(4.0),
                Phase::hold_full(4.0),
                Phase::exhale(4.0),
                Phase::hold_empty(4.0),
            ],
            intro: "Four equal sides: inhale, hold, exhale, hold.\nAll through the nose. Equal counts on every side.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "4-7-8 Breathing",
            purpose: "Rapid relaxation — good for anxiety and sleep",
            difficulty: Difficulty::Beginner,
            rounds: 4,
            phases: vec![
                Phase::inhale(4.0),
                Phase::hold_full(7.0),
                Phase::exhale_with(8.0, "through the mouth — audible release"),
            ],
            intro: "Inhale for 4, hold for 7, exhale for 8.\nThe long hold and extended exhale activate the parasympathetic system.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Extended Exhale",
            purpose: "Activate the vagus nerve — fast down-regulation",
            difficulty: Difficulty::Beginner,
            rounds: 10,
            phases: vec![
                Phase::inhale(5.0),
                Phase::exhale(10.0),
            ],
            intro: "Double the exhale length. In for 5 seconds, out for 10.\nA simple, reliable tool for calming the nervous system.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Nadi Shodhana",
            purpose: "Balance left and right hemispheres — center the mind",
            difficulty: Difficulty::Beginner,
            rounds: 10,
            phases: vec![
                Phase::inhale_with(5.0, "left nostril  (right thumb closes right)"),
                Phase::exhale_with(5.0, "right nostril  (ring finger closes left)"),
                Phase::inhale_with(5.0, "right nostril"),
                Phase::exhale_with(5.0, "left nostril  (right thumb closes right)"),
            ],
            intro: "Alternate nostril breathing.\nRight hand: thumb closes right nostril, ring finger closes left.\nInhale through one side, switch, exhale through the other.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Buteyko — Nasal Unblocking",
            purpose: "Clear a blocked nose in under 2 minutes, no medication",
            difficulty: Difficulty::Beginner,
            rounds: 3,
            phases: vec![
                Phase { label: "BREATHE", kind: PhaseKind::Inhale, duration_secs: 5.0, hint: Some("normal nasal breath in and out") },
                Phase::interactive("HOLD & WALK", "pinch your nose — walk 20-30 steps, sit down, then release"),
                Phase { label: "RECOVER", kind: PhaseKind::Inhale, duration_secs: 15.0, hint: Some("slow nasal breathing only") },
            ],
            intro: "A CO2 technique to open the nasal passage.\nAfter a normal exhale, pinch your nose and walk.\nRelease and breathe only through the nose.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Buteyko — Reduced Breathing",
            purpose: "Lower breathing volume; build CO₂ tolerance",
            difficulty: Difficulty::Intermediate,
            rounds: 8,
            phases: vec![
                Phase::inhale(4.0),
                Phase::exhale(4.0),
                Phase::interactive("HOLD", "gentle hold — stop at the first slight urge to breathe"),
                Phase { label: "RECOVER", kind: PhaseKind::Inhale, duration_secs: 8.0, hint: Some("slow nasal breathing — breathe a little less than usual") },
            ],
            intro: "Breathe gently through the nose — less than you normally would.\nAfter each exhale, hold until the first slight air hunger.\nGradually reduce your breathing volume each round.\n\nCP Test (optional, before you start):\nBreathe normally, then exhale. Pinch the nose and hold.\nCount seconds until the first distinct urge to breathe.\n40+ seconds: healthy.  Under 20: needs attention.",
            completion_note: None,
            warning: None,
        },
        Exercise {
            name: "Wim Hof — Bellows Breath",
            purpose: "Boost energy and immune resilience",
            difficulty: Difficulty::Advanced,
            rounds: 30,
            phases: vec![
                Phase::inhale(1.5),
                Phase::exhale(1.0),
            ],
            intro: "30 rounds of powerful bellows breathing.\nBreath hard in, let go — don't force the exhale.\nAfter 30 rounds: exhale fully, hold, then take one recovery breath.",
            completion_note: Some(
                "Exhale fully and hold your breath.\n\
                 When you must breathe: inhale fully, hold 15 seconds, then exhale.\n\
                 Repeat for 2-3 full cycles for a complete session.",
            ),
            warning: Some("Never near water or while driving. Dizziness is normal. Stop if faint."),
        },
        Exercise {
            name: "Tummo — Inner Fire",
            purpose: "Generate internal heat; shift the autonomic nervous system",
            difficulty: Difficulty::Advanced,
            rounds: 3,
            phases: vec![
                Phase { label: "BELLOWS", kind: PhaseKind::Inhale, duration_secs: 45.0, hint: Some("30 deep, powerful breaths — build intensity") },
                Phase::interactive("HOLD EMPTY", "exhale fully, hold — visualise heat rising from the belly"),
                Phase { label: "RECOVERY", kind: PhaseKind::HoldFull, duration_secs: 15.0, hint: Some("inhale fully, hold 15 seconds") },
                Phase::exhale(5.0),
            ],
            intro: "Three full cycles of inner fire breathing.\nEach round: 30 bellows breaths, hold on empty, then a recovery breath.\nVisualize warmth rising through the body on the hold.\nRooted in Tibetan Buddhist practice; popularised by Wim Hof.",
            completion_note: Some(
                "Rest for a few minutes.\n\
                 Notice any warmth, tingling, or aliveness in the body.\n\
                 This is a profound practice — give it time to settle.",
            ),
            warning: Some("Never near water, while driving, or standing. Tingling and dizziness are normal. Stop if faint."),
        },
        Exercise {
            name: "Holotropic Breathwork",
            purpose: "Deep psychological processing through sustained hyperventilation",
            difficulty: Difficulty::Advanced,
            rounds: 1,
            phases: vec![
                Phase::interactive("BREATHE DEEP", "faster and deeper than normal — find a rhythm and sustain it"),
                Phase::interactive("REST", "when ready — let the breath return to normal and stay still"),
            ],
            intro: "Sustained, deeper-than-normal breathing for an extended period.\nTraditionally done lying down with evocative music and a trained sitter.\nSessions typically run 2–3 hours; this is a guided introduction.\n\nLie down in a comfortable, safe space before you begin.",
            completion_note: Some(
                "Stay still and rest for as long as needed.\n\
                 This practice can surface strong emotion — give it space.",
            ),
            warning: Some("Best done with a trained facilitator. Avoid with cardiovascular conditions or psychiatric history."),
        },
        Exercise {
            name: "Sudarshan Kriya (SKY)",
            purpose: "Reduce cortisol; stabilise mood; documented PTSD relief",
            difficulty: Difficulty::Intermediate,
            rounds: 1,
            phases: vec![
                Phase { label: "SLOW  (20/min)", kind: PhaseKind::Inhale, duration_secs: 180.0, hint: Some("3 s in · 3 s out — deep nasal breaths") },
                Phase { label: "MEDIUM (40/min)", kind: PhaseKind::Inhale, duration_secs: 120.0, hint: Some("1.5 s in · 1.5 s out — steady rhythm") },
                Phase { label: "FAST  (120/min)", kind: PhaseKind::Inhale, duration_secs: 60.0,  hint: Some("rapid, powerful nasal rhythm") },
            ],
            intro: "Three stages of rhythmic nasal breathing, increasing in pace.\nFollow each stage's hint for the breathing cadence.\nThe bar and timer track your remaining time in each stage.\n\nBest learned in person from an instructor — this is a guide.",
            completion_note: Some(
                "Sit quietly for a few minutes.\n\
                 This practice measurably shifts cortisol and mood — let it settle.",
            ),
            warning: None,
        },
    ]
}
