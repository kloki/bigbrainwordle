pub const OPENING: (&str, &str) = (
    "Lets start with ",
    ". Put it in wordle's feedback with 'g' for 🟩, 'y' for 🟨 and ' ' for ⬜ Press enter to confirm.",
);

pub const CLOSING: (&str, &str) = ("Last change! Lets try ", ". 🤞");

pub const WON: (&str, &str) = (
    "🎉 Solved! The word was ",
    ". Press the 'ANY' key to leave.",
);

pub const LOST: &str = "Lost! 🤦 We ran out of words to suggest. Better luck next time!. Press the 'ANY' key to leave.";

pub const FAILED: &str = "👹 None the words I know match the feedback. Either we made a mistake or the word is not in my dictionary. Press the 'ANY' key to leave.";

pub const SUGGESTIONS: [(&str, &str); 20] = [
    ("Interesting! 🧐 Next, try ", "."),
    ("🔥 Now go with ", "."),
    ("Victory is close. Let's try ", ". 🥇"),
    ("Clear! Next try ", ". 🎯"),
    ("Hmm 🤔, let's see what ", " does."),
    ("Next ", ". We are on course 🧭"),
    ("Why don't we give ", " a shot? 🤷"),
    ("Let's test ", " and find out! 🚀"),
    ("My brain says ", ". Trust me. 🧠"),
    ("The stars align for ", ". ✨"),
    ("According to my calculations... ", ". 🤓"),
    ("I've got a feeling about ", "! 🎲"),
    ("Elementary, my dear Watson. Try ", ". 🔍"),
    ("The oracle whispers ", ". 🔮"),
    ("Big brain move: ", ". 💡"),
    ("Plot twist! Go with ", ". 🎬"),
    ("Science says ", ". Don't argue with science. 🧪"),
    ("I'd bet my circuits on ", ". 🤖"),
    ("Bold strategy: ", ". Let's see if it pays off. 🎰"),
    ("Chef's kiss. Try ", ". 👨‍🍳"),
];

pub fn suggestion_text(index: usize) -> (&'static str, &'static str) {
    SUGGESTIONS[index % SUGGESTIONS.len()]
}
