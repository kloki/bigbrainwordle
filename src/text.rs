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

pub const SUGGESTIONS: [(&str, &str); 8] = [
    ("Interesting! 🧐 Next, try ", "."),
    ("🔥 Now go with ", "."),
    ("Victory is close. Let's try ", ". 🥇"),
    ("Clear! Next try ", ". 🎯"),
    ("Hmm 🤔, let's see what ", " does."),
    ("Next ", ". We are on course 🧭"),
    ("Why don't we give ", " a shot? 🤷"),
    ("Let's test ", " and find out! 🚀"),
];

pub fn suggestion_text(index: usize) -> (&'static str, &'static str) {
    // This is a simple way to get a radnom suggestion but we the seem between renders.
    SUGGESTIONS[index % 8]
}
