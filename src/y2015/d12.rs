pub fn parse_input(input: &str) -> serde_json::Value {
	serde_json::from_str(input).unwrap()
}

fn sum_json(value: &serde_json::Value, ignore_key: &Option<String>) -> i64 {
	if let Some(obj) = value.as_object() {
		if let Some(key) = ignore_key
			&& obj.values().filter_map(|v| v.as_str()).any(|v| v == key)
		{
			return 0;
		}

		return obj.values().map(|v| sum_json(v, ignore_key)).sum();
	}

	if let Some(arr) = value.as_array() {
		return arr.iter().map(|v| sum_json(v, ignore_key)).sum();
	}

	if let Some(num) = value.as_i64() {
		return num;
	}

	0
}

pub fn part1(input: &serde_json::Value) -> i64 {
	sum_json(input, &None)
}

pub fn part2(input: &serde_json::Value) -> i64 {
	sum_json(input, &Some("red".to_string()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("[1,2,3]")), 6);
		assert_eq!(part1(&parse_input(r#"{"a":2,"b":4}"#)), 6);
		assert_eq!(part1(&parse_input("[[[3]]]")), 3);
		assert_eq!(part1(&parse_input(r#"{"a":{"b":4},"c":-1}"#)), 3);
		assert_eq!(part1(&parse_input(r#"{"a":[-1,1]}"#)), 0);
		assert_eq!(part1(&parse_input(r#"[-1,{"a":1}]"#)), 0);
		assert_eq!(part1(&parse_input("[]")), 0);
		assert_eq!(part1(&parse_input("{}")), 0);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input("[1,2,3]")), 6);
		assert_eq!(part2(&parse_input(r#"[1,{"c":"red","b":2},3]"#)), 4);
		assert_eq!(part2(&parse_input(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
		assert_eq!(part2(&parse_input(r#"[1,"red",5]"#)), 6);
	}
}
