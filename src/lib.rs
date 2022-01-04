use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - the tricky part is that the ball will not start
     until hit the wall or the bounds
     - so what does this means when you do a search (BFS or DFS)?
       - while you are visiting neighbors you need to go as far
         as you can in the specified direction until you hit
         a wall or bounds
       - only when the ball stopped the node it stopped at can
         be considered as visited; whatever nodes it roll passed
         can't be mark as visited or you will be going nowhere
         very soon
  - we use BFS so we need a queue
*/

impl Solution {
  pub fn has_path(start: &(usize, usize), dest: &(usize, usize), maze: &Vec<Vec<usize>>) -> bool {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    Self::bfs(start, dest, maze, &mut visited)
  }

  fn bfs(
    node: &(usize, usize),
    dest: &(usize, usize),
    maze: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
  ) -> bool {
    /*
      - right, left, down, up
    */
    let neighbors = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue = VecDeque::from([*node]);

    let rows = maze.len();
    let cols = maze[0].len();
    let (dest_x, dest_y) = dest;

    while queue.len() > 0 {
      /*
        - use size to make sure we have
          visited all nodes at one level
          before moving on to the next
      */
      let size = queue.len();

      for _ in 0..size {
        if let Some((node_x, node_y)) = queue.pop_front() {
          if node_x == *dest_x && node_y == *dest_y {
            return true;
          }
          for neighbor in &neighbors {
            let (x_move, y_move) = neighbor;
            let mut next_x_check = node_x as isize + x_move;
            let mut next_y_check = node_y as isize + y_move;

            /* make sure the next move is within bounds */
            while next_x_check >= 0
              && next_x_check < rows as isize
              && next_y_check >= 0
              && next_y_check < cols as isize
              && maze[next_x_check as usize][next_y_check as usize] == 0
            {
              /*
                - keep moving to a partiuclar direction
                - either x_move or y_move one of them will
                  be zero; so we will be just moving along
                  either x or y direction whichever the move
                  is not zero
              */
              next_x_check += x_move;
              next_y_check += y_move;
            }

            /*
              - we need to deduct one move back due to
                how we design the while loop
            */
            let next_x = (next_x_check - x_move) as usize;
            let next_y = (next_y_check - y_move) as usize;
            /*
              - to see if this node is our rightful candidate
                for the next round
            */
            if maze[next_x][next_y] == 0 && !visited[next_x][next_y] {
              visited[next_x][next_y] = true;
              queue.push_back((next_x, next_y));
            }
          }
        }
      }
    }

    false
  }

  pub fn test_fixture_1() -> Vec<Vec<usize>> {
    vec![
      vec![0, 0, 1, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 1, 0],
      vec![1, 1, 0, 1, 1],
      vec![0, 0, 0, 0, 0],
    ]
  }

  pub fn test_fixture_2() -> Vec<Vec<usize>> {
    vec![
      vec![0, 0, 1, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 1, 0],
      vec![1, 1, 0, 1, 1],
      vec![0, 0, 0, 0, 0],
    ]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::has_path(&(0, 4), &(4, 4), &Solution::test_fixture_1());
    assert_eq!(result, true);
  }

  #[test]
  fn sample_2() {
    let result = Solution::has_path(&(0, 4), &(3, 2), &Solution::test_fixture_2());
    assert_eq!(result, false);
  }
}
