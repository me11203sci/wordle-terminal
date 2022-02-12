#include <fstream>
#include <iostream>
#include <ctime>
#include <vector>
#include <regex>

using namespace std;

string getRandomWord()
{
	srand(time(0));
	
	ifstream file("words.txt");
	vector<string> listOfWords;

	for(string line; getline(file, line);)
	{
		listOfWords.push_back(line);
	}

	return listOfWords[rand() % (listOfWords.size())]; 
}

int main()
{
	string word = getRandomWord(), userGuess;

	// Program start blurb.
	cout << "================================WORDLE================================";
	cout << "\n\nYou have six tries to guess a random five letter word, with\neach guess revealing information about the answer." << endl;
	cout << "\nFor example, if the answer were \"cramp\" and you guessed\n\"cheer\":";
	cout << "\n\n---+---+---+---+---\n c | h | e | e | r \n---+---+---+---+---\n G | B | B | B | Y \n---+---+---+---+---\n";
	cout << "\nWhere:\n(G)reen - Letter is both in the answer and in the correct position." << endl;
	cout << "(Y)ellow - Letter is in the answer but in the wrong position." << endl;
	cout << "(B)lack - Letter is not in the word." << endl;
	cout << "\nWith that said, guess a five letter word:\n>> "; 

	for(int guesses = 5; guesses >= 0; guesses--) 
	{
		cin >> userGuess;
		while(!regex_match(userGuess, regex("^[A-Za-z]{5}$")))
		{
			cout << "\nInvalid guess. Try again.\n>> ";
			cin >> userGuess;
		}

		vector<vector<char>> output = {{'\0', '\0', '\0', '\0', '\0'}};
		vector<char> guessAsVector(userGuess.begin(), userGuess.end());
		vector<char> information;
		for(int i = 0; i < 5; i++) information.push_back((userGuess[i] == word[i])? 'G': (word.find(userGuess[i]) != string::npos)? 'Y': 'B');			

		output.push_back(guessAsVector);
		output.push_back(information);
		
		for(int i = 0; i < 3; i++)
		{
			for(int j = 0; j < 5; j++) cout << ' ' << output[i][j] << ((i != 0)? ((j != 4)? " |" : "") : "");
			cout << "\n---+---+---+---+---\n";
		}
		if(userGuess == word)
		{
			cout << "\n===============================YOU WIN!===============================" << endl;
			return 0;
		}
		if(guesses != 0) cout << "\nYou have " << guesses <<" guesses left. Try again.\n>> ";
	}
	cout << "\nThe word was \"" << word << "\".\n===============================YOU LOSE!===============================" << endl;
	return 0;
}
