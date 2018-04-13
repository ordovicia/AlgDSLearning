#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;

struct Player {
    string name;
    int score;
}; 

bool cmp_str(const string& s1, const string& s2) {
    for (auto ch1 = s1.cbegin(), ch2 = s2.cbegin(); ; ch1++, ch2++) {
        if (ch1 == s1.cend()) {
            if (ch2 == s2.cend())
                return false;
            else
                return true;
        } else if (ch2 == s2.cend()) {
            return false;
        }
        
        if (*ch1 == *ch2) 
            continue;
        return *ch1 < *ch2;
    }
}

bool cmp_player(const Player& p1, const Player& p2) {
    if (p1.score == p2.score)
        return cmp_str(p1.name, p2.name);
    else
        return p1.score > p2.score;
}

vector<Player> comparator(vector<Player> players) {
    sort(players.begin(), players.end(), cmp_player);
    return players;
}

int main() {
    
    int n;
    cin >> n;
    vector< Player > players;
    string name;
    int score;
    for(int i = 0; i < n; i++){
        cin >> name >> score;
        Player p1;
        p1.name = name, p1.score = score;
        players.push_back(p1);
    }
    
    vector<Player > answer = comparator(players);
    for(int i = 0; i < answer.size(); i++) {
        cout << answer[i].name << " " << answer[i].score << endl;
    }
    return 0;
}