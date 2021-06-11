const keywords = ["print"];

export class TokenizerError extends Error {
  index: number;
  constructor(message: string, index: number) {
    super(message);
    this.index = index;
  }
}

// returns a token if the given regex matches at the current index
const regexMatcher =
  (regex: string, type: TokenType): Matcher =>
  (input: string, index: number) => {
    const match = input.substring(index).match(regex);
    return (
      match && {
        type,
        value: match[0],
      }
    );
  };

const matchers = [
  regexMatcher("^[.0-9+", "number"),
  regexMatcher(`^(${keywords.join("|")})`, "keyword"),
  regexMatcher("^\\s+", "whitespace"),
];

const locationForIndex = (input: string, index: number) => ({
  char: index - input.lastIndexOf("\n", index) - 1,
  line: input.substring(0, index).split("\n").length - 1,
});

export const tokenize: Tokenizer = (input) => {
  const tokens: Token[] = [];
  let index = 0;
  while (index < input.length) {
    const matches = matchers
      .map((m) => m(input, index))
      .filter((f): f is Token => !!f);
    // take the first priroity match
    const match = matches[0];
    if (match.type !== "whitespace") {
      tokens.push({ ...match, ...locationForIndex(input, index) });
    } else {
      throw new TokenizerError(
        `Unexpected token ${input.substring(index, index + 1)}`,
        index
      );
    }
    index += match.value.length;
  }
  return tokens;
};
