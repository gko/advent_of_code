// to launch node --stack-size=10000 -- $NVM_BIN/ts-node ~/test.ts
const calculateScore = (cards: Array<number>) =>
	cards.reduce((acc, card, idx) => acc + card * (cards.length - idx), 0);

const game = (
	player1: Array<number> = [],
	player2: Array<number> = [],
	subgame = false,
	previousRounds: Array<string> = []
): Array<number> | number => {
	const currentRound = JSON.stringify({ player1, player2 });

	if (previousRounds.indexOf(currentRound) >= 0) {
		return 1;
	}

	previousRounds.push(currentRound);

	if (player1.length === 0 || player2.length === 0) {
		if (subgame) {
			return player1.length ? 1 : 2;
		}

		return calculateScore(player1.length ? player1 : player2);
	}

	const cardPlayer1 = player1.shift();
	const cardPlayer2 = player2.shift();

	if (!cardPlayer2 || !cardPlayer1) {
		return calculateScore(player1.length ? player1 : player2);
	}

	let winner = null;
	if (player1.length >= cardPlayer1 && player2.length >= cardPlayer2) {
		winner = game(
			player1.slice(0, cardPlayer1),
			player2.slice(0, cardPlayer2),
			true,
			[]
		) as number;
	}

	if (winner === 2 || (winner === null && cardPlayer2 > cardPlayer1)) {
		return game(
			player1,
			[...player2, cardPlayer2, cardPlayer1],
			subgame,
			previousRounds
		);
	}

	return game(
		[...player1, cardPlayer1, cardPlayer2],
		player2,
		subgame,
		previousRounds
	);
};

console.log(
	game(
		[
			7,
			1,
			9,
			10,
			12,
			4,
			38,
			22,
			18,
			3,
			27,
			31,
			43,
			33,
			47,
			42,
			21,
			24,
			50,
			39,
			8,
			6,
			16,
			46,
			11,
		],
		[
			49,
			41,
			40,
			35,
			44,
			29,
			30,
			19,
			14,
			2,
			34,
			17,
			25,
			5,
			15,
			32,
			20,
			48,
			45,
			26,
			37,
			28,
			36,
			23,
			13,
		]
	)
);

export {};
