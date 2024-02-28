/*
 * Author(s): Melesio Albavera
 * Created: 24 February 2024
 * Description: Command-line interface implementation of popular online game.
 */
use chrono::offset::Local;
use cursive::{
    event::{Event, EventResult, Key::{
        Backspace,
        Enter,
    }}, theme::{Color, ColorStyle}, view::CannotFocus, views::{Button, Dialog, LinearLayout, PaddedView, ResizedView}, Cursive, Printer, Vec2
};
use itertools::multizip;
use regex::Regex;
use serde_json::Value; 
use std::{collections::HashMap, error::Error, fs};

struct BoardView
{
    board: [char; 30],
    board_index: usize,
    guesses: u8,
    information: [u8; 30],
    message: String,
    solution: String,
    won: bool,
}

struct InstructionView {}

impl BoardView
{
    pub fn new() -> Self
    {
        BoardView
        {
            board: [' '; 30],
            board_index: 0,
            guesses: 0,
            information: [0; 30],
            message: "".to_string(),
            solution: fetch_solution().unwrap().to_uppercase().replace("\"", ""),
            won: false,
        }
    }
}

impl InstructionView
{
    pub fn new() -> Self
    {
        InstructionView {}
    }
}

impl cursive::view::View for BoardView
{
    fn draw(&self, printer: &Printer)
    {
        let printer = &printer.offset((9, 0));
        let mut text_color: Color = Color::Rgb(0, 0, 0);
        let mut fill_color: Color = Color::Rgb(208, 207, 204);

        for y in 0..6
        {
            for x in 0..5
            {
                match self.information[y * 5 + x]
                {
                    3 => { text_color = Color::Rgb(255, 255, 255); fill_color = Color::Rgb(83, 141, 78) }
                    2 => { text_color = Color::Rgb(255, 255, 255); fill_color = Color::Rgb(182, 159, 59) }
                    1 => { text_color = Color::Rgb(255, 255, 255); fill_color = Color::Rgb(58, 58, 60) }
                    0 => { text_color = Color::Rgb(0, 0, 0); fill_color = Color::Rgb(208, 207, 204) }
                    _ => {}
                }
                printer.print(((25 - self.message.len()) / 2, 0), &self.message);
                printer.with_color
                (
                    ColorStyle::new(text_color, fill_color),
                    |p|
                    {
                        p.print((x * 5, y * 3 + 1), "┌───┐");
                        p.print((x * 5, y * 3 + 2), &vec!["│ ", &self.board[y * 5 + x].to_string(), " │"].join(""));
                        p.print((x * 5, y * 3 + 3), "└───┘");
                    }
                );
            }
        }
    }

    fn take_focus(&mut self, _source: cursive::direction::Direction) -> Result<EventResult, CannotFocus>
    {
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: Event) -> EventResult
    {
        if self.board_index == 30 || self.won || self.guesses > 6 
        {
            return EventResult::Ignored
        }

        match event
        {
            Event::Char(c) =>
            {
                if Regex::new(r"[A-Za-z]").unwrap().is_match(&c.to_string())
                {
                    self.message = "".to_string();
                    if self.board[self.board_index] == ' ' 
                    {
                        self.board[self.board_index] = c.to_uppercase().next().unwrap();
                    }
                    if self.board_index % 5 != 4 && self.board_index != 29
                    { 
                        self.board_index += 1; 
                    }
                }
                return EventResult::Consumed(None);
            }
            Event::Key(k) =>
            {
                if k == Backspace
                {
                    if self.board_index % 5 != 0 && self.board[self.board_index] == ' '
                    {
                        self.board_index -= 1; 
                    }
                    self.board[self.board_index] = ' ';
                    return EventResult::Consumed(None);
                }
                else if k == Enter
                {
                    if self.board_index % 5 == 4 && self.board[self.board_index] != ' '
                    {
                        self.board_index += 1;
                        let guess: String = self.board[self.board_index - 5 .. self.board_index].iter().collect::<String>();
                        if verify(guess.clone())
                        {
                            let result: Vec<u8> = compare(guess.clone(), self.solution.clone());
                            self.information[self.board_index - 5 .. self.board_index].copy_from_slice(&result);
                            self.guesses += 1;
                            if result == vec![3u8; 5]
                            {
                                self.won = true;
                                match self.guesses
                                {
                                    1 => self.message = "Genius".to_string(),
                                    2 => self.message = "Magnificent".to_string(),
                                    3 => self.message = "Impressive".to_string(),
                                    4 => self.message = "Splendid".to_string(),
                                    5 => self.message = "Great".to_string(),
                                    6 => self.message = "Phew".to_string(),
                                    _ => self.message = "ERROR".to_string(),
                                }
                            }
                            else
                            {
                                if self.guesses == 6
                                {
                                    self.message = self.solution.clone()
                                }
                            }
                        }
                        else 
                        {
                            self.message = "Not in word list".to_string();
                            self.board_index -= 1;
                        }
                    }
                    return EventResult::Consumed(None);
                }
            }
            _ => ()
        }
        EventResult::Ignored
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2
    {
        Vec2::new(25, 19)
    }
}

impl cursive::view::View for InstructionView
{
    fn draw(&self, printer: &Printer)
    {
        printer.print((0, 0), "Guess the Wordle in 6 tries.");
        printer.print((0, 1), "  - Each guess must be a valid 5-letter word.");
        printer.print((0, 2), "  - The color of the tiles will change to show how close your");
        printer.print((0, 3), "    guess was to the word.");
        printer.print((0, 4), "Examples");
        printer.with_color
        (
            ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(83, 141, 78)),
            |p|
            {
                p.print((0, 5), "┌───┐");
                p.print((0, 6), "│ W │");
                p.print((0, 7), "└───┘");
            }
        );
        printer.print((5, 5), "┌───┐┌───┐┌───┐┌───┐");
        printer.print((5, 6), "│ E ││ A ││ R ││ Y │");
        printer.print((5, 7), "└───┘└───┘└───┘└───┘");
        printer.print((0, 8), "W is in the word and in the correct spot.");
        printer.print((0, 9), "┌───┐     ┌───┐┌───┐┌───┐");
        printer.print((0, 10), "│ P │     │ L ││ L ││ S │");
        printer.print((0, 11), "└───┘     └───┘└───┘└───┘");
        printer.with_color
        (
            ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(182, 159, 59)),
            |p|
            {
                p.print((5, 9), "┌───┐");
                p.print((5, 10), "│ I │");
                p.print((5, 11), "└───┘");
            }
        );
        printer.print((0, 12), "I is in the word but in the wrong spot.");
        printer.print((0, 13), "┌───┐┌───┐┌───┐     ┌───┐");
        printer.print((0, 14), "│ V ││ A ││ G │     │ E │");
        printer.print((0, 15), "└───┘└───┘└───┘     └───┘");
        printer.with_color
        (
            ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(58, 58, 60)),
            |p|
            {
                p.print((15, 13), "┌───┐");
                p.print((15, 14), "│ U │");
                p.print((15, 15), "└───┘");
            }
        );
        printer.print((0, 16), "U is not in the word in any spot.");
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2
    {
        Vec2::new(61, 17)
    }
}

fn verify(s: String) -> bool
{
    let file_string: String = fs::read_to_string("./src/words.txt").unwrap();
    let words: Vec<&str> = file_string.split('\n').collect::<Vec<&str>>();
    match words.binary_search(&s.clone().as_str())
    {
        Ok(_) => true,
        _ => false,
    }
}

fn compare(guess: String, solution: String) -> Vec<u8>
{
    let mut result: Vec<u8> = Vec::from([0, 0, 0, 0, 0]);

    let mut count: HashMap<char, u8> = HashMap::new();
    for c in solution.chars() { *count.get_mut(&c).unwrap() = *count.entry(c).or_insert(0) + 1; }

    for (g, s, r) in multizip((guess.chars(), solution.chars(), &mut result))
    {
        if g == s
        {
            *r = 3;
            *count.get_mut(&g).unwrap() -= 1;
        }
        else if count.contains_key(&g) && *count.get(&g).unwrap() != 0
        {
            *r = 2;
            *count.get_mut(&g).unwrap() -= 1;
        }
        else { *r = 1 }
    }

    return result;
}

fn fetch_solution() -> Result<String, Box<dyn Error>>
{
    let url: String = vec!["https://www.nytimes.com/svc/wordle/v2/".to_string(), Local::now().date_naive().to_string(), ".json".to_string()].join("");

    let solution: String = reqwest::blocking::get(url)?
        .json::<HashMap<String, Value>>()?
        .get("solution").unwrap().to_string()
        .to_uppercase();

    Ok(solution)
}

fn instructions(r: &mut Cursive)
{
    r.add_layer
    (
        Dialog::new()
            .title("How To Play")
            .content(InstructionView::new())   
            .dismiss_button("Ok"),
    );
}

fn main()
{
    let mut root = cursive::default();
    let buttons = LinearLayout::horizontal()
        .child(PaddedView::lrtb(11, 1, 0, 0, Button::new("How To Play", instructions)))
        // .child(PaddedView::lrtb(0, 1, 0, 0, Button::new("Statistics", |r| ())))
        // .child(PaddedView::lrtb(0, 1, 0, 0, Button::new("Settings", |r| ())))
        .child(Button::new("Quit", |r| r.quit()));
    
    root.add_layer(
        Dialog::new()
            .content(
                ResizedView::with_fixed_width(43,
                    LinearLayout::vertical()
                        .child(
                            BoardView::new()
                        )
                        .child(buttons)
                )
            )
        .title("WORDLE")
    );

    root.run();
}
