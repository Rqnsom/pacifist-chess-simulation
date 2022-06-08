# Pacifist Chess Simulation

`🙃 Living person 1`: Have you ever wondered what a game of chess would look like if it were played by two pacifists?

`😐 Living person 2`: What? 

`🙃 Living person 1`: **Exactly!**

`💀 Person 3`: 💀

`😐 Living person 2`: Does that make any sense?

`🙃 Living person 1`: Well, yes! It's probably fun! Or... mostly boring! **Please check out these animations below.**

<hr \>

## Player modes
Simulation actually supports four *AI modes*. Although AI is not the right word here, since there is no smart logic implemented.
It's all random! But with a few small factors taken in mind. We can call it *RandomAI*, if you like.
 - **Pacifist** - Whenever possible, choose _non-capture_ moves! But do prioritize non-capture _checkmate_ turns! The only reason *capture* move could be played is if there is no other possible move to be played - such scenario happens more often when king is in check.
 - **Aggressive** - Always prefer _capture/check_ turns. Only the _checkmate_ turn has a higher priorty.
 - **SemiPacifist** - Pacifist mode where pawns are allowed to capture other pawns. We can call it _promotion party mode_.
 - **Random** - Turn is decided completely randomly from the pool of all available turns.

<hr \>

### Pacifist player vs Pacifist player

In the first game below, **the black player wins with `Qb3#`**.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/p_vs_p_black_won_Qb3%23.gif" align="center" height=auto width=100% border="black"></a>

In the second game below, the game ends in a **draw** (50 move rule).

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/p_vs_p_draw_50_move_rule.gif" align="center" height=auto width=100% border="black"></a>

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/p_vs_p.png" align="center" height=auto width=100% border="black"></a>
<hr \>

### Pacifist player vs Aggressive player
We can assume such a game will not last very long.

In the example below, the aggressive player was black, and it won the game in 18 moves.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/p_vs_a_black_won_Ng4.gif" align="center" height=auto width=100% border="black"></a>

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/p_vs_a.png" align="center" height=auto width=100% border="black"></a>

<hr \>

### Aggressive player vs Aggressive player

Two random AI players that both prefer *capture* turns? Would such a game end quickly? What would be the outcome?

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/a_vs_a_mate_not_possible_Kxe5.gif" align="center" height=auto width=100% border="black"></a>

Well. Unfortunately, most games with aggressive players end up with a draw. Too aggressive and too dumb to be able to win the game more often.
<hr \>

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/a_vs_a.png" align="center" height=auto width=100% border="black"></a>

### SemiPacifist player vs SemiPacifist player

This mode has some quite interesting stats!

`Example #1`: The black player wins with `Qg3#` on the 123rd move.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/sp_vs_sp_black_won_Qg3.gif" align="center" height=auto width=100% border="black"></a>

`Example #2`: The white player wins with `b8=Q#` on the 107th move.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/sp_vs_sp_white_won_b8Q.gif" align="center" height=auto width=100% border="black"></a>

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/sp_vs_sp.png" align="center" height=auto width=100% border="black"></a>

<hr \>

### Random player vs Random player

Two fully random players have the most randomized results. Which kind of makes sense.

`Example #1`: On the 111th move, this game ends in a **stalemate**.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/r_vs_r_stalemate_Qf3.gif" align="center" height=auto width=100% border="black"></a>

`Example #2`: The white player wins with `Qa7#` on the 93th move.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/r_vs_r_white_won_Qa7.gif" align="center" height=auto width=100% border="black"></a>

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/pacifist-chess-simulation/main/assets/r_vs_r.png" align="center" height=auto width=100% border="black"></a>

<hr \>

## About
This is how I decided to practice my Rust skills. This small experiment was done for, well, mostly fun... I believe! So enjoy these silly statistics and have fun!

### Dependencies
These two Rust crates below were created just for this project, but now could be reused for something more useful. Less absurd.
 - [chess-notation-parser](https://github.com/Rqnsom/chess-notation-parser)
 - [chess-turn-engine](https://github.com/Rqnsom/chess-turn-engine)

> But why didn't you use any existing chess tools or libraries for this funny project?

That was not the point! The fastest route is not always the best one. The main goal was to learn Rust! And doing the whole chess thing from scratch was an interesting challenge and a jolly adventure. 🙃
