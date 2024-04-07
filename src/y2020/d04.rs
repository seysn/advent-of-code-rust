use std::collections::HashMap;

struct ParsePassportError;

trait PassportElement: Sized {
	fn parse(value: &str) -> Result<Self, ParsePassportError>;
	fn is_valid(&self) -> bool {
		true
	}
}

struct BirthYear(u32);
struct IssueYear(u32);
struct ExpirationYear(u32);
enum Height {
	Cm(u32),
	In(u32),
}
struct HairColor(String);
struct EyeColor(String);
struct PassportId(String);

struct Passport {
	byr: BirthYear,
	iyr: IssueYear,
	eyr: ExpirationYear,
	hgt: Height,
	hcl: HairColor,
	ecl: EyeColor,
	pid: PassportId,
}

impl PassportElement for BirthYear {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.parse().map_err(|_| ParsePassportError)?))
	}

	fn is_valid(&self) -> bool {
		(1920..=2002).contains(&self.0)
	}
}

impl PassportElement for IssueYear {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.parse().map_err(|_| ParsePassportError)?))
	}

	fn is_valid(&self) -> bool {
		(2010..=2020).contains(&self.0)
	}
}

impl PassportElement for ExpirationYear {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.parse().map_err(|_| ParsePassportError)?))
	}

	fn is_valid(&self) -> bool {
		(2020..=2030).contains(&self.0)
	}
}

impl PassportElement for Height {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		if let Some(v) = value.strip_suffix("cm") {
			Ok(Self::Cm(v.parse().map_err(|_| ParsePassportError)?))
		} else if let Some(v) = value.strip_suffix("in") {
			Ok(Self::In(v.parse().map_err(|_| ParsePassportError)?))
		} else {
			Err(ParsePassportError)
		}
	}

	fn is_valid(&self) -> bool {
		match self {
			Height::Cm(v) => (150..=193).contains(v),
			Height::In(v) => (59..=76).contains(v),
		}
	}
}

impl PassportElement for HairColor {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.to_string()))
	}

	fn is_valid(&self) -> bool {
		if let Some(hex) = self.0.strip_prefix('#') {
			u32::from_str_radix(hex, 16).is_ok()
		} else {
			false
		}
	}
}

impl PassportElement for EyeColor {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.to_string()))
	}

	fn is_valid(&self) -> bool {
		["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.0.as_ref())
	}
}

impl PassportElement for PassportId {
	fn parse(value: &str) -> Result<Self, ParsePassportError> {
		Ok(Self(value.to_string()))
	}

	fn is_valid(&self) -> bool {
		if self.0.len() != 9 {
			return false;
		}

		self.0.parse::<u32>().is_ok()
	}
}

impl TryFrom<&HashMap<String, String>> for Passport {
	type Error = ParsePassportError;

	fn try_from(value: &HashMap<String, String>) -> Result<Self, Self::Error> {
		let byr = if let Some(v) = value.get("byr") {
			BirthYear::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let iyr = if let Some(v) = value.get("iyr") {
			IssueYear::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let eyr = if let Some(v) = value.get("eyr") {
			ExpirationYear::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let hgt = if let Some(v) = value.get("hgt") {
			Height::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let hcl = if let Some(v) = value.get("hcl") {
			HairColor::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let ecl = if let Some(v) = value.get("ecl") {
			EyeColor::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		let pid = if let Some(v) = value.get("pid") {
			PassportId::parse(v)?
		} else {
			return Err(ParsePassportError);
		};

		Ok(Self {
			byr,
			ecl,
			eyr,
			hcl,
			hgt,
			iyr,
			pid,
		})
	}
}

impl Passport {
	fn is_valid(&self) -> bool {
		self.byr.is_valid()
			&& self.iyr.is_valid()
			&& self.eyr.is_valid()
			&& self.hgt.is_valid()
			&& self.hcl.is_valid()
			&& self.ecl.is_valid()
			&& self.pid.is_valid()
	}
}

pub fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
	input
		.split("\n\n")
		.map(|p| {
			p.replace('\n', " ")
				.split(' ')
				.map(|s| {
					let (a, b) = s.split_once(':').unwrap();
					(a.to_string(), b.to_string())
				})
				.collect()
		})
		.collect()
}

pub fn part1(input: &[HashMap<String, String>]) -> usize {
	let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	input.iter().filter(|p| fields.iter().all(|f| p.contains_key(*f))).count()
}

pub fn part2(input: &[HashMap<String, String>]) -> usize {
	input
		.iter()
		.filter_map(|m| Passport::try_from(m).ok())
		.filter(|p| p.is_valid())
		.count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 2);
	}
}
