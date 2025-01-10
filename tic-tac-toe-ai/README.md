
# Tic-Tac-Toe AI with Minimax Algorithm

This project implements a Tic-Tac-Toe game with an AI opponent using the **Minimax Algorithm**. The AI is designed to make optimal moves, and a weighted version of the algorithm is included to prioritize strategic board positions.

---

## Minimax Algorithm

### **What is Minimax?**
The Minimax Algorithm is a decision-making algorithm used in two-player games like Tic-Tac-Toe, Chess, and Checkers. It explores all possible moves and their outcomes to ensure that a player makes the best possible move while assuming the opponent is also playing optimally.

### **History of Minimax**
The Minimax Algorithm was introduced in the mid-20th century by researchers studying game theory, particularly by John von Neumann and Oskar Morgenstern in their foundational work, *Theory of Games and Economic Behavior* (1944). Minimax became a cornerstone of artificial intelligence in gaming, powering early computer programs designed to play games like Chess and Checkers.

### **How It Works**
The algorithm evaluates moves by simulating all potential future states of the game. It assigns a "score" to each state:
- **Positive scores** represent favorable outcomes for the AI.
- **Negative scores** represent favorable outcomes for the opponent.
- **Zero** indicates a draw.

The AI assumes that the opponent will always make the best possible counter-move, and it minimizes the opponent's potential gain while maximizing its own.

---

## Implementation Details

This project implements two versions of the Minimax Algorithm:
1. **Standard Minimax**
2. **Weighted Minimax** (a heuristic-enhanced version)

### **Shortcomings of Minimax**
While Minimax guarantees optimal play:
- **Computationally Expensive**: It evaluates all possible game states, which grows exponentially with the game’s complexity.
- **No Adaptation**: Minimax lacks any notion of "learning" or adaptation. It follows the same logic for every game, regardless of past experiences.
- **Uniform Decision Making**: Without additional heuristics, Minimax cannot differentiate between equally valued states (e.g., choosing between two equally good moves).

---

## Weighted Minimax

### **Why Add Weights?**
To address the limitations of standard Minimax, weights are introduced to prioritize strategic positions on the board:
- **Center**: Provides control over multiple winning lines.
- **Corners**: Useful for creating and blocking winning opportunities.
- **Edges**: Less critical but still useful in certain scenarios.

Weights guide the AI to:
1. Favor moves that control strategic positions.
2. Make decisions that are tactically advantageous early in the game.

### **Why Weights Only Slightly Help**
Adding weights improves the AI's decision-making but does not constitute "learning" because:
- **Static Heuristics**: The weights are predefined and do not change based on gameplay outcomes.
- **No Memory**: The algorithm doesn’t adapt its strategy based on previous games.
- **Prioritization, Not Knowledge**: Weights influence the AI’s preferences but do not provide it with an understanding of why a move is good or bad beyond the predefined logic.

---

## How to Run

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd tic-tac-toe-ai
   ```
2. Build and run the project with Cargo:
   - **Standard Minimax:**
     ```bash
     cargo run
     ```
   - **Weighted Minimax:**
     ```bash
     cargo run -- --w
     ```

3. Play the game by entering your moves in the format `row col` (e.g., `1 1`).

---

## Project Structure

- **`main.rs`**: Entry point for the game. Handles user input and game flow.
- **`board.rs`**: Defines the Tic-Tac-Toe board and its operations.
- **`game_state.rs`**: Manages game state and player-related logic.
- **`minimax.rs`**: Implements the standard Minimax Algorithm.
- **`minimax_weighted.rs`**: Implements the Weighted Minimax Algorithm.
- **`knowledge_cache.rs`**: Handles caching of evaluated board states for optimization.
- **`user_interface.rs`**: Handles user interactions and displays the board.

---

## Future Improvements

1. **Machine Learning**: Integrate a learning algorithm to allow the AI to adapt its strategy over time.
2. **Dynamic Weights**: Adjust weights based on game state or historical performance.
3. **Alpha-Beta Pruning**: Optimize the Minimax search to reduce unnecessary evaluations.
4. **Neural Networks**: Use neural networks to evaluate board states dynamically.

---

## Conclusion
This project demonstrates the fundamentals of game AI with Minimax and heuristics. While effective for simple games like Tic-Tac-Toe, it highlights the importance of more advanced techniques like learning and pruning for complex games.

Feel free to explore, modify, and extend the project to deepen your understanding of AI and game theory!
