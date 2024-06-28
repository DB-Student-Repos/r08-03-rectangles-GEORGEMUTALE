use std::collections::HashSet;

pub fn count(lines: &[&str]) -> u32 {
    let mut corners: HashSet<(usize, usize)> = HashSet::new();
    let mut rectangle_count: u32 = 0;

    // Iterate over each line and character in the input
    for (y, line) in lines.iter().enumerate() {
        // Filter and process only the '+' characters
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '+') {
            // Find smaller corners (top-left) relative to the current corner
            let smaller_corners = corners.iter()
                .filter(|(other_x, other_y)| x > *other_x && y > *other_y);

            for (top_left_x, top_left_y) in smaller_corners {
                // Check for the presence of the other two corners (bottom-left and top-right)
                if corners.contains(&(*top_left_x, y)) && corners.contains(&(x, *top_left_y)) {
                    // Verify that all sides of the rectangle are intact
                    if line_between(lines, &(*top_left_x, y), &(x, y)) &&
                       line_between(lines, &(x, *top_left_y), &(x, y)) &&
                       line_between(lines, &(*top_left_x, *top_left_y), &(*top_left_x, y)) &&
                       line_between(lines, &(*top_left_x, *top_left_y), &(x, *top_left_y)) {
                        rectangle_count += 1;
                    }
                }
            }

            // Add the current corner to the set of corners
            corners.insert((x, y));
        }
    }

    rectangle_count
}

fn line_between(lines: &[&str], first_point: &(usize, usize), second_point: &(usize, usize)) -> bool {
    let is_horizontal_line = |c: char| c == '-' || c == '+';
    let is_vertical_line = |c: char| c == '|' || c == '+';

    // Check if the line is horizontal
    if first_point.0 < second_point.0 {
        lines[first_point.1][first_point.0+1..second_point.0].chars().all(is_horizontal_line)
    } else {
        // Otherwise, check if the line is vertical
        lines[first_point.1+1..second_point.1].iter()
            .map(|str| str.chars().nth(first_point.0).unwrap())
            .all(is_vertical_line)
    }
}