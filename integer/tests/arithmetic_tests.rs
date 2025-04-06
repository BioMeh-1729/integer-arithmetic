use integer::Int;

#[test]
fn from_string() {
	assert_eq!(Int::zero(), Int::from_str("0"));
	assert_eq!(Int::one(), Int::from_str("1"));

	let arr1 = vec![16909288046191928514, 13214532208089606103];
	let num1 = "311921309056738720207868695253354784727";
	let int1 = Int{ size: 2, sign: false, value: arr1};

	assert_eq!(int1, Int::from_str(num1));
	assert_eq!(-int1, Int::from_str(&("-".to_owned() + num1)));

	let arr2 = vec![16950676358112219419, 7140599369499950061, 14076428321230156719];
	let num2 = "5768016272049219157445004099497874510885971505532193469359";
	let int2 = Int{ size: 3, sign: false, value: arr2};

	assert_eq!(int2, Int::from_str(num2));
	assert_eq!(-int2, Int::from_str(&("-".to_owned() + num2)));
}

#[test]
fn adder() {
	let zero = Int::zero();
	let one = Int::one();
	let two = &one + &one;
	let three = &two + &one;
	let five = &two + &three;
	let eight = &three + &five;
	let thirteen = &eight + &five;
	assert_eq!(Int::from_str("0"), zero);
	assert_eq!(Int::from_str("1"), one);
	assert_eq!(Int::from_str("2"), two);
	assert_eq!(Int::from_str("3"), three);
	assert_eq!(Int::from_str("5"), five);
	assert_eq!(Int::from_str("8"), eight);
	assert_eq!(Int::from_str("13"), thirteen);
}

#[test]
fn multiplication() {
	let zero = Int::zero();
	let one = Int::one();
	let three = &one * 3;
	assert_eq!(Int::from_str("3"), three);
	assert_eq!(&three * 1, three);
	assert_eq!(&three * &one, three);
	let seventeen = Int::from_str("17");
	assert_eq!(seventeen * three, Int::from_str("51"));

	let arr1 = vec![16909288046191928514, 13214532208089606103];
	let sq_1 = "97294903043669512759665593417923829623618342134903140890603489474064512464529";
	let int1 = Int{ size: 2, sign: false, value: arr1};

	let arr2 = vec![16950676358112219419, 7140599369499950061, 14076428321230156719];
	let sq_2 = "33270011714624571786076249707480490518884337222881163780997955528849510520544123539261605801461559119418368871870881";
	let int2 = Int{ size: 3, sign: false, value: arr2};

	let mul3 = "1799167186238162413432357262889641946087504547425795280605626923336526946271542782502564715679993";
	assert_eq!(&int1 * &int1, Int::from_str(sq_1));
	assert_eq!(&int2 * &int2, Int::from_str(sq_2));
	assert_eq!(&int1 * &int2, Int::from_str(mul3));
}