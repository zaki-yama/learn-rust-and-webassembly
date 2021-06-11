export class ParserError extends Error {
  token: Token;
  constructor(message: string, token: Token) {
    super(message);
    this.token = token;
  }
}

export const parse: Parser = (tokens) => {
  const iterator = tokens[Symbol.iterator]();
  let currentToken: Token = iterator.next().value;

  const eatToken = () => (currentToken = iterator.next().value);

  //

  const parseStatement = ()=>{
  //   if (currentToken.type==='keyword'){
  //     switch(currentToken.value)
  // const nodes: StatementNode[] = [];
  //   }
  while (index < tokens.length) {
    nodes.push(parseStatement());
  }

  return nodes;
};
