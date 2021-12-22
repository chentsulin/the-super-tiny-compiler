#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi_derive::napi;
use napi::bindgen_prelude::*;
use regex::Regex;

/**
 * TTTTTTTTTTTTTTTTTTTTTTTHHHHHHHHH     HHHHHHHHHEEEEEEEEEEEEEEEEEEEEEE
 * T:::::::::::::::::::::TH:::::::H     H:::::::HE::::::::::::::::::::E
 * T:::::::::::::::::::::TH:::::::H     H:::::::HE::::::::::::::::::::E
 * T:::::TT:::::::TT:::::THH::::::H     H::::::HHEE::::::EEEEEEEEE::::E
 * TTTTTT  T:::::T  TTTTTT  H:::::H     H:::::H    E:::::E       EEEEEE
 *         T:::::T          H:::::H     H:::::H    E:::::E
 *         T:::::T          H::::::HHHHH::::::H    E::::::EEEEEEEEEE
 *         T:::::T          H:::::::::::::::::H    E:::::::::::::::E
 *         T:::::T          H:::::::::::::::::H    E:::::::::::::::E
 *         T:::::T          H::::::HHHHH::::::H    E::::::EEEEEEEEEE
 *         T:::::T          H:::::H     H:::::H    E:::::E
 *         T:::::T          H:::::H     H:::::H    E:::::E       EEEEEE
 *       TT:::::::TT      HH::::::H     H::::::HHEE::::::EEEEEEEE:::::E
 *       T:::::::::T      H:::::::H     H:::::::HE::::::::::::::::::::E
 *       T:::::::::T      H:::::::H     H:::::::HE::::::::::::::::::::E
 *       TTTTTTTTTTT      HHHHHHHHH     HHHHHHHHHEEEEEEEEEEEEEEEEEEEEEE
 *
 *    SSSSSSSSSSSSSSS UUUUUUUU     UUUUUUUUPPPPPPPPPPPPPPPPP   EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRR
 *  SS:::::::::::::::SU::::::U     U::::::UP::::::::::::::::P  E::::::::::::::::::::ER::::::::::::::::R
 * S:::::SSSSSS::::::SU::::::U     U::::::UP::::::PPPPPP:::::P E::::::::::::::::::::ER::::::RRRRRR:::::R
 * S:::::S     SSSSSSSUU:::::U     U:::::UUPP:::::P     P:::::PEE::::::EEEEEEEEE::::ERR:::::R     R:::::R
 * S:::::S             U:::::U     U:::::U   P::::P     P:::::P  E:::::E       EEEEEE  R::::R     R:::::R
 * S:::::S             U:::::U     U:::::U   P::::P     P:::::P  E:::::E               R::::R     R:::::R
 *  S::::SSSS          U:::::U     U:::::U   P::::PPPPPP:::::P   E::::::EEEEEEEEEE     R::::RRRRRR:::::R
 *   SS::::::SSSSS     U:::::U     U:::::U   P:::::::::::::PP    E:::::::::::::::E     R:::::::::::::RR
 *     SSS::::::::SS   U:::::U     U:::::U   P::::PPPPPPPPP      E:::::::::::::::E     R::::RRRRRR:::::R
 *        SSSSSS::::S  U:::::U     U:::::U   P::::P              E::::::EEEEEEEEEE     R::::R     R:::::R
 *             S:::::S U:::::U     U:::::U   P::::P              E:::::E               R::::R     R:::::R
 *             S:::::S U::::::U   U::::::U   P::::P              E:::::E       EEEEEE  R::::R     R:::::R
 * SSSSSSS     S:::::S U:::::::UUU:::::::U PP::::::PP          EE::::::EEEEEEEE:::::ERR:::::R     R:::::R
 * S::::::SSSSSS:::::S  UU:::::::::::::UU  P::::::::P          E::::::::::::::::::::ER::::::R     R:::::R
 * S:::::::::::::::SS     UU:::::::::UU    P::::::::P          E::::::::::::::::::::ER::::::R     R:::::R
 *  SSSSSSSSSSSSSSS         UUUUUUUUU      PPPPPPPPPP          EEEEEEEEEEEEEEEEEEEEEERRRRRRRR     RRRRRRR
 *
 * TTTTTTTTTTTTTTTTTTTTTTTIIIIIIIIIINNNNNNNN        NNNNNNNNYYYYYYY       YYYYYYY
 * T:::::::::::::::::::::TI::::::::IN:::::::N       N::::::NY:::::Y       Y:::::Y
 * T:::::::::::::::::::::TI::::::::IN::::::::N      N::::::NY:::::Y       Y:::::Y
 * T:::::TT:::::::TT:::::TII::::::IIN:::::::::N     N::::::NY::::::Y     Y::::::Y
 * TTTTTT  T:::::T  TTTTTT  I::::I  N::::::::::N    N::::::NYYY:::::Y   Y:::::YYY
 *         T:::::T          I::::I  N:::::::::::N   N::::::N   Y:::::Y Y:::::Y
 *         T:::::T          I::::I  N:::::::N::::N  N::::::N    Y:::::Y:::::Y
 *         T:::::T          I::::I  N::::::N N::::N N::::::N     Y:::::::::Y
 *         T:::::T          I::::I  N::::::N  N::::N:::::::N      Y:::::::Y
 *         T:::::T          I::::I  N::::::N   N:::::::::::N       Y:::::Y
 *         T:::::T          I::::I  N::::::N    N::::::::::N       Y:::::Y
 *         T:::::T          I::::I  N::::::N     N:::::::::N       Y:::::Y
 *       TT:::::::TT      II::::::IIN::::::N      N::::::::N       Y:::::Y
 *       T:::::::::T      I::::::::IN::::::N       N:::::::N    YYYY:::::YYYY
 *       T:::::::::T      I::::::::IN::::::N        N::::::N    Y:::::::::::Y
 *       TTTTTTTTTTT      IIIIIIIIIINNNNNNNN         NNNNNNN    YYYYYYYYYYYYY
 *
 *         CCCCCCCCCCCCC     OOOOOOOOO     MMMMMMMM               MMMMMMMMPPPPPPPPPPPPPPPPP   IIIIIIIIIILLLLLLLLLLL             EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRR
 *      CCC::::::::::::C   OO:::::::::OO   M:::::::M             M:::::::MP::::::::::::::::P  I::::::::IL:::::::::L             E::::::::::::::::::::ER::::::::::::::::R
 *    CC:::::::::::::::C OO:::::::::::::OO M::::::::M           M::::::::MP::::::PPPPPP:::::P I::::::::IL:::::::::L             E::::::::::::::::::::ER::::::RRRRRR:::::R
 *   C:::::CCCCCCCC::::CO:::::::OOO:::::::OM:::::::::M         M:::::::::MPP:::::P     P:::::PII::::::IILL:::::::LL             EE::::::EEEEEEEEE::::ERR:::::R     R:::::R
 *  C:::::C       CCCCCCO::::::O   O::::::OM::::::::::M       M::::::::::M  P::::P     P:::::P  I::::I    L:::::L                 E:::::E       EEEEEE  R::::R     R:::::R
 * C:::::C              O:::::O     O:::::OM:::::::::::M     M:::::::::::M  P::::P     P:::::P  I::::I    L:::::L                 E:::::E               R::::R     R:::::R
 * C:::::C              O:::::O     O:::::OM:::::::M::::M   M::::M:::::::M  P::::PPPPPP:::::P   I::::I    L:::::L                 E::::::EEEEEEEEEE     R::::RRRRRR:::::R
 * C:::::C              O:::::O     O:::::OM::::::M M::::M M::::M M::::::M  P:::::::::::::PP    I::::I    L:::::L                 E:::::::::::::::E     R:::::::::::::RR
 * C:::::C              O:::::O     O:::::OM::::::M  M::::M::::M  M::::::M  P::::PPPPPPPPP      I::::I    L:::::L                 E:::::::::::::::E     R::::RRRRRR:::::R
 * C:::::C              O:::::O     O:::::OM::::::M   M:::::::M   M::::::M  P::::P              I::::I    L:::::L                 E::::::EEEEEEEEEE     R::::R     R:::::R
 * C:::::C              O:::::O     O:::::OM::::::M    M:::::M    M::::::M  P::::P              I::::I    L:::::L                 E:::::E               R::::R     R:::::R
 *  C:::::C       CCCCCCO::::::O   O::::::OM::::::M     MMMMM     M::::::M  P::::P              I::::I    L:::::L         LLLLLL  E:::::E       EEEEEE  R::::R     R:::::R
 *   C:::::CCCCCCCC::::CO:::::::OOO:::::::OM::::::M               M::::::MPP::::::PP          II::::::IILL:::::::LLLLLLLLL:::::LEE::::::EEEEEEEE:::::ERR:::::R     R:::::R
 *    CC:::::::::::::::C OO:::::::::::::OO M::::::M               M::::::MP::::::::P          I::::::::IL::::::::::::::::::::::LE::::::::::::::::::::ER::::::R     R:::::R
 *      CCC::::::::::::C   OO:::::::::OO   M::::::M               M::::::MP::::::::P          I::::::::IL::::::::::::::::::::::LE::::::::::::::::::::ER::::::R     R:::::R
 *         CCCCCCCCCCCCC     OOOOOOOOO     MMMMMMMM               MMMMMMMMPPPPPPPPPP          IIIIIIIIIILLLLLLLLLLLLLLLLLLLLLLLLEEEEEEEEEEEEEEEEEEEEEERRRRRRRR     RRRRRRR
 *
 * =======================================================================================================================================================================
 * =======================================================================================================================================================================
 * =======================================================================================================================================================================
 * =======================================================================================================================================================================
 */

/**
 * Today we're going to write a compiler together. But not just any compiler... A
 * super duper teeny tiny compiler! A compiler that is so small that if you
 * remove all the comments this file would only be ~200 lines of actual code.
 *
 * We're going to compile some lisp-like function calls into some C-like
 * function calls.
 *
 * If you are not familiar with one or the other. I'll just give you a quick intro.
 *
 * If we had two functions `add` and `subtract` they would be written like this:
 *
 *                  LISP                      C
 *
 *   2 + 2          (add 2 2)                 add(2, 2)
 *   4 - 2          (subtract 4 2)            subtract(4, 2)
 *   2 + (4 - 2)    (add 2 (subtract 4 2))    add(2, subtract(4, 2))
 *
 * Easy peezy right?
 *
 * Well good, because this is exactly what we are going to compile. While this
 * is neither a complete LISP or C syntax, it will be enough of the syntax to
 * demonstrate many of the major pieces of a modern compiler.
 */

/**
 * Most compilers break down into three primary stages: Parsing, Transformation,
 * and Code Generation
 *
 * 1. *Parsing* is taking raw code and turning it into a more abstract
 *    representation of the code.
 *
 * 2. *Transformation* takes this abstract representation and manipulates to do
 *    whatever the compiler wants it to.
 *
 * 3. *Code Generation* takes the transformed representation of the code and
 *    turns it into new code.
 */

/**
 * Parsing
 * -------
 *
 * Parsing typically gets broken down into two phases: Lexical Analysis and
 * Syntactic Analysis.
 *
 * 1. *Lexical Analysis* takes the raw code and splits it apart into these things
 *    called tokens by a thing called a tokenizer (or lexer).
 *
 *    Tokens are an array of tiny little objects that describe an isolated piece
 *    of the syntax. They could be numbers, labels, punctuation, operators,
 *    whatever.
 *
 * 2. *Syntactic Analysis* takes the tokens and reformats them into a
 *    representation that describes each part of the syntax and their relation
 *    to one another. This is known as an intermediate representation or
 *    Abstract Syntax Tree.
 *
 *    An Abstract Syntax Tree, or AST for short, is a deeply nested object that
 *    represents code in a way that is both easy to work with and tells us a lot
 *    of information.
 *
 * For the following syntax:
 *
 *   (add 2 (subtract 4 2))
 *
 * Tokens might look something like this:
 *
 *   [
 *     { type: 'paren',  value: '('        },
 *     { type: 'name',   value: 'add'      },
 *     { type: 'number', value: '2'        },
 *     { type: 'paren',  value: '('        },
 *     { type: 'name',   value: 'subtract' },
 *     { type: 'number', value: '4'        },
 *     { type: 'number', value: '2'        },
 *     { type: 'paren',  value: ')'        },
 *     { type: 'paren',  value: ')'        },
 *   ]
 *
 * And an Abstract Syntax Tree (AST) might look like this:
 *
 *   {
 *     type: 'Program',
 *     body: [{
 *       type: 'CallExpression',
 *       name: 'add',
 *       params: [{
 *         type: 'NumberLiteral',
 *         value: '2',
 *       }, {
 *         type: 'CallExpression',
 *         name: 'subtract',
 *         params: [{
 *           type: 'NumberLiteral',
 *           value: '4',
 *         }, {
 *           type: 'NumberLiteral',
 *           value: '2',
 *         }]
 *       }]
 *     }]
 *   }
 */

/**
 * Transformation
 * --------------
 *
 * The next type of stage for a compiler is transformation. Again, this just
 * takes the AST from the last step and makes changes to it. It can manipulate
 * the AST in the same language or it can translate it into an entirely new
 * language.
 *
 * Let’s look at how we would transform an AST.
 *
 * You might notice that our AST has elements within it that look very similar.
 * There are these objects with a type property. Each of these are known as an
 * AST Node. These nodes have defined properties on them that describe one
 * isolated part of the tree.
 *
 * We can have a node for a "NumberLiteral":
 *
 *   {
 *     type: 'NumberLiteral',
 *     value: '2',
 *   }
 *
 * Or maybe a node for a "CallExpression":
 *
 *   {
 *     type: 'CallExpression',
 *     name: 'subtract',
 *     params: [...nested nodes go here...],
 *   }
 *
 * When transforming the AST we can manipulate nodes by
 * adding/removing/replacing properties, we can add new nodes, remove nodes, or
 * we could leave the existing AST alone and create an entirely new one based
 * on it.
 *
 * Since we’re targeting a new language, we’re going to focus on creating an
 * entirely new AST that is specific to the target language.
 *
 * Traversal
 * ---------
 *
 * In order to navigate through all of these nodes, we need to be able to
 * traverse through them. This traversal process goes to each node in the AST
 * depth-first.
 *
 *   {
 *     type: 'Program',
 *     body: [{
 *       type: 'CallExpression',
 *       name: 'add',
 *       params: [{
 *         type: 'NumberLiteral',
 *         value: '2'
 *       }, {
 *         type: 'CallExpression',
 *         name: 'subtract',
 *         params: [{
 *           type: 'NumberLiteral',
 *           value: '4'
 *         }, {
 *           type: 'NumberLiteral',
 *           value: '2'
 *         }]
 *       }]
 *     }]
 *   }
 *
 * So for the above AST we would go:
 *
 *   1. Program - Starting at the top level of the AST
 *   2. CallExpression (add) - Moving to the first element of the Program's body
 *   3. NumberLiteral (2) - Moving to the first element of CallExpression's params
 *   4. CallExpression (subtract) - Moving to the second element of CallExpression's params
 *   5. NumberLiteral (4) - Moving to the first element of CallExpression's params
 *   6. NumberLiteral (2) - Moving to the second element of CallExpression's params
 *
 * If we were manipulating this AST directly, instead of creating a separate AST,
 * we would likely introduce all sorts of abstractions here. But just visiting
 * each node in the tree is enough for what we're trying to do.
 *
 * The reason I use the word "visiting" is because there is this pattern of how
 * to represent operations on elements of an object structure.
 *
 * Visitors
 * --------
 *
 * The basic idea here is that we are going to create a “visitor” object that
 * has methods that will accept different node types.
 *
 *   var visitor = {
 *     NumberLiteral() {},
 *     CallExpression() {},
 *   };
 *
 * When we traverse our AST, we will call the methods on this visitor whenever we
 * "enter" a node of a matching type.
 *
 * In order to make this useful we will also pass the node and a reference to
 * the parent node.
 *
 *   var visitor = {
 *     NumberLiteral(node, parent) {},
 *     CallExpression(node, parent) {},
 *   };
 *
 * However, there also exists the possibility of calling things on "exit". Imagine
 * our tree structure from before in list form:
 *
 *   - Program
 *     - CallExpression
 *       - NumberLiteral
 *       - CallExpression
 *         - NumberLiteral
 *         - NumberLiteral
 *
 * As we traverse down, we're going to reach branches with dead ends. As we
 * finish each branch of the tree we "exit" it. So going down the tree we
 * "enter" each node, and going back up we "exit".
 *
 *   -> Program (enter)
 *     -> CallExpression (enter)
 *       -> Number Literal (enter)
 *       <- Number Literal (exit)
 *       -> Call Expression (enter)
 *          -> Number Literal (enter)
 *          <- Number Literal (exit)
 *          -> Number Literal (enter)
 *          <- Number Literal (exit)
 *       <- CallExpression (exit)
 *     <- CallExpression (exit)
 *   <- Program (exit)
 *
 * In order to support that, the final form of our visitor will look like this:
 *
 *   var visitor = {
 *     NumberLiteral: {
 *       enter(node, parent) {},
 *       exit(node, parent) {},
 *     }
 *   };
 */

/**
 * Code Generation
 * ---------------
 *
 * The final phase of a compiler is code generation. Sometimes compilers will do
 * things that overlap with transformation, but for the most part code
 * generation just means take our AST and string-ify code back out.
 *
 * Code generators work several different ways, some compilers will reuse the
 * tokens from earlier, others will have created a separate representation of
 * the code so that they can print nodes linearly, but from what I can tell most
 * will use the same AST we just created, which is what we’re going to focus on.
 *
 * Effectively our code generator will know how to “print” all of the different
 * node types of the AST, and it will recursively call itself to print nested
 * nodes until everything is printed into one long string of code.
 */

/**
 * And that's it! That's all the different pieces of a compiler.
 *
 * Now that isn’t to say every compiler looks exactly like I described here.
 * Compilers serve many different purposes, and they might need more steps than
 * I have detailed.
 *
 * But now you should have a general high-level idea of what most compilers look
 * like.
 *
 * Now that I’ve explained all of this, you’re all good to go write your own
 * compilers right?
 *
 * Just kidding, that's what I'm here to help with :P
 *
 * So let's begin...
 */

/**
 * ============================================================================
 *                                   (/^▽^)/
 *                                THE TOKENIZER!
 * ============================================================================
 */

/**
 * We're gonna start off with our first phase of parsing, lexical analysis, with
 * the tokenizer.
 *
 * We're just going to take our string of code and break it down into an array
 * of tokens.
 *
 *   (add 2 (subtract 4 2))   =>   [{ type: 'paren', value: '(' }, ...]
 */

#[napi]
fn tokenizer(env: Env, input: String) -> Result<Vec<Object>> {
    // A `current` variable for tracking our position in the code like a cursor.
    let mut current = 0;

    // And a `tokens` array for pushing our tokens to.
    let mut tokens = Vec::new();

    // We start by creating a `while` loop where we are setting up our `current`
    // variable to be incremented as much as we want `inside` the loop.
    //
    // We do this because we may want to increment `current` many times within a
    // single loop because our tokens can be any length.
    while current < input.len() {
        // We're also going to store the `current` character in the `input`.
        let mut char = input.chars().nth(current).unwrap();

        // The first thing we want to check for is an open parenthesis. This will
        // later be used for `CallExpression` but for now we only care about the
        // character.
        //
        // We check to see if we have an open parenthesis:
        if char == '(' {
            // If we do, we push a new token with the type `paren` and set the value
            // to an open parenthesis.
            let mut token = env.create_object().unwrap();
            token.set("type", "paren").unwrap();
            token.set("value", "(").unwrap();
            tokens.push(token);

            // Then we increment `current`
            current += 1;

            // And we `continue` onto the next cycle of the loop.
            continue;
        }

        // Next we're going to check for a closing parenthesis. We do the same exact
        // thing as before: Check for a closing parenthesis, add a new token,
        // increment `current`, and `continue`.
        if char == ')' {
            let mut token = env.create_object().unwrap();
            token.set("type", "paren").unwrap();
            token.set("value", ")").unwrap();
            tokens.push(token);
            current += 1;
            continue;
        }

        // Moving on, we're now going to check for whitespace. This is interesting
        // because we care that whitespace exists to separate characters, but it
        // isn't actually important for us to store as a token. We would only throw
        // it out later.
        //
        // So here we're just going to test for existence and if it does exist we're
        // going to just `continue` on.
        let whitespace = Regex::new(r"\s").unwrap();
        if whitespace.is_match(&char.to_string()) {
            current += 1;
            continue;
        }

        // The next type of token is a number. This is different than what we have
        // seen before because a number could be any number of characters and we
        // want to capture the entire sequence of characters as one token.
        //
        //   (add 123 456)
        //        ^^^ ^^^
        //        Only two separate tokens
        //
        // So we start this off when we encounter the first number in a sequence.
        let numbers = Regex::new(r"[0-9]").unwrap();
        if numbers.is_match(&char.to_string()) {
            // We're going to create a `value` string that we are going to push
            // characters to.
            let mut value = String::from("");

            // Then we're going to loop through each character in the sequence until
            // we encounter a character that is not a number, pushing each character
            // that is a number to our `value` and incrementing `current` as we go.
            while numbers.is_match(&char.to_string()) {
                value = format!("{}{}", value, char);
                current += 1;
                char = input.chars().nth(current).unwrap();
            }

            // After that we push our `number` token to the `tokens` array.
            let mut token = env.create_object().unwrap();
            token.set("type", "number").unwrap();
            token.set("value", value).unwrap();
            tokens.push(token);

            // And we continue on.
            continue;
        }

        // We'll also add support for strings in our language which will be any
        // text surrounded by double quotes (").
        //
        //   (concat "foo" "bar")
        //            ^^^   ^^^ string tokens
        //
        // We'll start by checking for the opening quote:
        if char == '"' {
            // Keep a `value` variable for building up our string token.
            let mut value = String::from("");

            // We'll skip the opening double quote in our token.
            current += 1;
            char = input.chars().nth(current).unwrap();

            // Then we'll iterate through each character until we reach another
            // double quote.
            while char != '"' {
                value = format!("{}{}", value, char);
                current += 1;
                char = input.chars().nth(current).unwrap();
            }

            // Skip the closing double quote.
            current += 1;

            // And add our `string` token to the `tokens` array.
            let mut token = env.create_object().unwrap();
            token.set("type", "string").unwrap();
            token.set("value", value).unwrap();
            tokens.push(token);

            continue;
        }

        // The last type of token will be a `name` token. This is a sequence of
        // letters instead of numbers, that are the names of functions in our lisp
        // syntax.
        //
        //   (add 2 4)
        //    ^^^
        //    Name token
        //
        let letters = Regex::new(r"(?i)[a-z]").unwrap();
        if letters.is_match(&char.to_string()) {
            let mut value = String::from("");

            // Again we're just going to loop through all the letters pushing them to
            // a value.
            while letters.is_match(&char.to_string()) {
                value = format!("{}{}", value, char);
                current += 1;
                char = input.chars().nth(current).unwrap();
            }

            // And pushing that value as a token with the type `name` and continuing.
            let mut token = env.create_object().unwrap();
            token.set("type", "name").unwrap();
            token.set("value", value).unwrap();
            tokens.push(token);

            continue;
        }

        // Finally if we have not matched a character by now, we're going to throw
        // an error and completely exit.
        return Err(Error {
            status: Status::GenericFailure,
            reason: format!("I dont know what this character is: {}", char),
        })
    }

    // Then at the end of our `tokenizer` we simply return the tokens array.
    Ok(tokens)
}

fn walk(env: &Env, tokens: &JsObject, current: &mut u32) -> Result<JsObject> {
    // Inside the walk function we start by grabbing the `current` token.
    let mut token = tokens.get_element::<JsObject>(*current)?;

    // We're going to split each type of token off into a different code path,
    // starting off with `number` tokens.
    //
    // We test to see if we have a `number` token.
    if token
        .get_named_property::<JsString>("type")?
        .into_utf8()?
        .as_str()?
        == "number"
    {
        // If we have one, we'll increment `current`.
        *current += 1;

        // And we'll return a new AST node called `NumberLiteral` and setting its
        // value to the value of our token.
        let mut result = env.create_object()?;
        result.set_named_property("type", env.create_string("NumberLiteral")?)?;
        result.set_named_property("value", token.get_named_property::<JsString>("value")?)?;
        return Ok(result);
    }

    // If we have a string we will do the same as number and create a
    // `StringLiteral` node.
    if token
        .get_named_property::<JsString>("type")?
        .into_utf8()?
        .as_str()?
        == "string"
    {
        *current += 1;

        let mut result = env.create_object()?;
        result.set_named_property("type", env.create_string("StringLiteral")?)?;
        result.set_named_property("value", token.get_named_property::<JsString>("value")?)?;
        return Ok(result);
    }

    // Next we're going to look for CallExpressions. We start this off when we
    // encounter an open parenthesis.
    if token
        .get_named_property::<JsString>("type")?
        .into_utf8()?
        .as_str()?
        == "paren"
        && token
            .get_named_property::<JsString>("value")?
            .into_utf8()?
            .as_str()?
            == "("
    {
        // We'll increment `current` to skip the parenthesis since we don't care
        // about it in our AST.
        *current += 1;
        token = tokens.get_element::<JsObject>(*current)?;

        // We create a base node with the type `CallExpression`, and we're going
        // to set the name as the current token's value since the next token after
        // the open parenthesis is the name of the function.
        let mut node = env.create_object()?;
        let mut params = env.create_array()?;
        node.set_named_property("type", env.create_string("CallExpression")?)?;
        node.set_named_property("name", token.get_named_property::<JsString>("value")?)?;
        node.set_named_property("params", &params)?;

        // We increment `current` *again* to skip the name token.
        *current += 1;
        token = tokens.get_element::<JsObject>(*current)?;

        // And now we want to loop through each token that will be the `params` of
        // our `CallExpression` until we encounter a closing parenthesis.
        //
        // Now this is where recursion comes in. Instead of trying to parse a
        // potentially infinitely nested set of nodes we're going to rely on
        // recursion to resolve things.
        //
        // To explain this, let's take our Lisp code. You can see that the
        // parameters of the `add` are a number and a nested `CallExpression` that
        // includes its own numbers.
        //
        //   (add 2 (subtract 4 2))
        //
        // You'll also notice that in our tokens array we have multiple closing
        // parenthesis.
        //
        //   [
        //     { type: 'paren',  value: '('        },
        //     { type: 'name',   value: 'add'      },
        //     { type: 'number', value: '2'        },
        //     { type: 'paren',  value: '('        },
        //     { type: 'name',   value: 'subtract' },
        //     { type: 'number', value: '4'        },
        //     { type: 'number', value: '2'        },
        //     { type: 'paren',  value: ')'        }, <<< Closing parenthesis
        //     { type: 'paren',  value: ')'        }, <<< Closing parenthesis
        //   ]
        //
        // We're going to rely on the nested `walk` function to increment our
        // `current` variable past any nested `CallExpression`.

        // So we create a `while` loop that will continue until it encounters a
        // token with a `type` of `'paren'` and a `value` of a closing
        // parenthesis.
        while (token
            .get_named_property::<JsString>("type")?
            .into_utf8()?
            .as_str()?
            != "paren")
            || (token
                .get_named_property::<JsString>("type")?
                .into_utf8()?
                .as_str()?
                == "paren"
                && token
                    .get_named_property::<JsString>("value")?
                    .into_utf8()?
                    .as_str()?
                    != ")")
        {
            // we'll call the `walk` function which will return a `node` and we'll
            // push it into our `node.params`.
            params.set_element(params.get_array_length()?, walk(env, tokens, current)?)?;
            token = tokens.get_element::<JsObject>(*current)?;
        }

        // Finally we will increment `current` one last time to skip the closing
        // parenthesis.
        *current += 1;

        // And return the node.
        return Ok(node);
    }

    // Again, if we haven't recognized the token type by now we're going to
    // throw an error.
    Err(Error {
        status: Status::GenericFailure,
        reason: token
            .get_named_property::<JsString>("type")?
            .into_utf8()?
            .as_str()?
            .to_string(),
    })
}

/**
 * ============================================================================
 *                                 ヽ/❀o ل͜ o\ﾉ
 *                                THE PARSER!!!
 * ============================================================================
 */

/**
 * For our parser we're going to take our array of tokens and turn it into an
 * AST.
 *
 *   [{ type: 'paren', value: '(' }, ...]   =>   { type: 'Program', body: [...] }
 */

// Okay, so we define a `parser` function that accepts our array of `tokens`.

#[napi]
fn parser(env: Env, tokens: Array) -> Object {
    // Again we keep a `current` variable that we will use as a cursor.
    let mut current = 0;

    // Now, we're going to create our AST which will have a root which is a
    // `Program` node.
    let mut ast = env.create_object().unwrap();
    let mut body = env.create_array().unwrap();
    ast.set("type", "Program").unwrap();
    ast.set("body", &body).unwrap();

    // And we're going to kickstart our `walk` function, pushing nodes to our
    // `ast.body` array.
    //
    // The reason we are doing this inside a loop is because our program can have
    // `CallExpression` after one another instead of being nested.
    //
    //   (add 2 2)
    //   (subtract 4 2)
    //
    while current < tokens.get_array_length()? {
        body.set_element(
            body.get_array_length()?,
            walk(env, &tokens, &mut current)?,
        )?;
    }

    ast
}

/**
 * ============================================================================
 *                                 ⌒(❀>◞౪◟<❀)⌒
 *                               THE TRAVERSER!!!
 * ============================================================================
 */

/**
 * So now we have our AST, and we want to be able to visit different nodes with
 * a visitor. We need to be able to call the methods on the visitor whenever we
 * encounter a node with a matching type.
 *
 *   traverse(ast, {
 *     Program: {
 *       enter(node, parent) {
 *         // ...
 *       },
 *       exit(node, parent) {
 *         // ...
 *       },
 *     },
 *
 *     CallExpression: {
 *       enter(node, parent) {
 *         // ...
 *       },
 *       exit(node, parent) {
 *         // ...
 *       },
 *     },
 *
 *     NumberLiteral: {
 *       enter(node, parent) {
 *         // ...
 *       },
 *       exit(node, parent) {
 *         // ...
 *       },
 *     },
 *   });
 */

// So we define a traverser function which accepts an AST and a
// visitor. Inside we're going to define two functions...

#[js_function(2)]
fn traverser(ctx: CallContext) -> Result<JsUndefined> {
    let ast = ctx.get::<JsObject>(0)?;
    let visitor = ctx.get::<JsObject>(1)?;

    // Finally we kickstart the traverser by calling `traverse_node` with our ast
    // with no `parent` because the top level of the AST doesn't have a parent.
    traverse_node(&ast, None, &visitor)?;

    ctx.env.get_undefined()
}

// A `traverse_array` function that will allow us to iterate over an array and
// call the next function that we will define: `traverse_node`.
fn traverse_array(
  array: &JsObject,
  parent: &JsObject,
  visitor: &JsObject,
) -> Result<()> {
  let len = array.get_array_length()?;
  for index in 0..len {
      traverse_node(
          &array.get_element::<JsObject>(index)?,
          Some(parent),
          visitor,
      )?;
  }

  Ok(())
}

// `traverse_node` will accept a `node` and its `parent` node. So that it can
// pass both to our visitor methods.
fn traverse_node(
  node: &JsObject,
  parent: Option<&JsObject>,
  visitor: &JsObject,
) -> Result<()> {
  // We start by testing for the existence of a method on the visitor with a
  // matching `type`.
  let node_type = node
      .get_named_property::<JsString>("type")?
      .into_utf8()?
      .as_str()?
      .to_string();

  if visitor.has_named_property(&node_type)? {
      let methods = visitor.get_named_property::<JsObject>(&node_type)?;

      // If there is an `enter` method for this node type we'll call it with the
      // `node` and its `parent`.
      if visitor.has_named_property(&node_type)? && methods.has_named_property("enter")? {
          methods
              .get_named_property::<JsFunction>("enter")?
              .call(None, &[node, parent.unwrap()])?;
      }
  }

  // Next we are going to split things up by the current node type.
  match node_type.as_str() {
      // We'll start with our top level `Program`. Since Program nodes have a
      // property named body that has an array of nodes, we will call
      // `traverse_array` to traverse down into them.
      //
      // (Remember that `traverse_array` will in turn call `traverse_node` so  we
      // are causing the tree to be traversed recursively)
      "Program" => {
          traverse_array(&node.get_named_property("body")?, node, visitor)?;
      }

      // Next we do the same with `CallExpression` and traverse their `params`.
      "CallExpression" => {
          traverse_array(&node.get_named_property("params")?, node, visitor)?;
      }

      // In the cases of `NumberLiteral` and `StringLiteral` we don't have any
      // child nodes to visit, so we'll just break.
      "NumberLiteral" => (),
      "StringLiteral" => (),

      // And again, if we haven't recognized the node type then we'll throw an
      // error.
      _ => return Err(Error {
          status: Status::GenericFailure,
          reason: node_type,
      })
  }

  // If there is an `exit` method for this node type we'll call it with the
  // `node` and its `parent`.
  if visitor.has_named_property(&node_type)? {
      let methods = visitor.get_named_property::<JsObject>(&node_type)?;

      if visitor.has_named_property(&node_type)? && methods.has_named_property("exit")? {
          methods
              .get_named_property::<JsFunction>("exit")?
              .call(None, &[node, parent.unwrap()])?;
      }
  }

  Ok(())
}


/**
 * ============================================================================
 *                                   ⁽(◍˃̵͈̑ᴗ˂̵͈̑)⁽
 *                              THE TRANSFORMER!!!
 * ============================================================================
 */

/**
 * Next up, the transformer. Our transformer is going to take the AST that we
 * have built and pass it to our traverser function with a visitor and will
 * create a new ast.
 *
 * ----------------------------------------------------------------------------
 *   Original AST                     |   Transformed AST
 * ----------------------------------------------------------------------------
 *   {                                |   {
 *     type: 'Program',               |     type: 'Program',
 *     body: [{                       |     body: [{
 *       type: 'CallExpression',      |       type: 'ExpressionStatement',
 *       name: 'add',                 |       expression: {
 *       params: [{                   |         type: 'CallExpression',
 *         type: 'NumberLiteral',     |         callee: {
 *         value: '2'                 |           type: 'Identifier',
 *       }, {                         |           name: 'add'
 *         type: 'CallExpression',    |         },
 *         name: 'subtract',          |         arguments: [{
 *         params: [{                 |           type: 'NumberLiteral',
 *           type: 'NumberLiteral',   |           value: '2'
 *           value: '4'               |         }, {
 *         }, {                       |           type: 'CallExpression',
 *           type: 'NumberLiteral',   |           callee: {
 *           value: '2'               |             type: 'Identifier',
 *         }]                         |             name: 'subtract'
 *       }]                           |           },
 *     }]                             |           arguments: [{
 *   }                                |             type: 'NumberLiteral',
 *                                    |             value: '4'
 * ---------------------------------- |           }, {
 *                                    |             type: 'NumberLiteral',
 *                                    |             value: '2'
 *                                    |           }]
 *  (sorry the other one is longer.)  |         }
 *                                    |       }
 *                                    |     }]
 *                                    |   }
 * ----------------------------------------------------------------------------
 */

// So we have our transformer function which will accept the lisp ast.

#[napi]
fn transformer(ctx: CallContext) -> Result<JsObject> {
    let mut ast = ctx.get::<JsObject>(0)?;

    // We'll create a `newAst` which like our previous AST will have a program
    // node.
    let mut new_ast = ctx.env.create_object()?;
    let body = ctx.env.create_array()?;
    new_ast.set_named_property("type", ctx.env.create_string("Program")?)?;
    new_ast.set_named_property("body", &body)?;

    // Next I'm going to cheat a little and create a bit of a hack. We're going to
    // use a property named `context` on our parent nodes that we're going to push
    // nodes to their parent's `context`. Normally you would have a better
    // abstraction than this, but for our purposes this keeps things simple.
    //
    // Just take note that the context is a reference *from* the old ast *to* the
    // new ast.
    ast.set_named_property("_context", &body)?;

    let mut visitor = ctx.env.create_object()?;

    // The first visitor method accepts any `NumberLiteral`
    let mut number_literal = ctx.env.create_object()?;
    visitor.set_named_property("NumberLiteral", &number_literal)?;
    // We'll visit them on enter.
    number_literal.set_named_property(
        "enter",
        ctx.env.create_function("enter", on_number_literal_enter)?,
    )?;

    // Next we have `StringLiteral`
    let mut string_literal = ctx.env.create_object()?;
    visitor.set_named_property("StringLiteral", &string_literal)?;
    string_literal.set_named_property(
        "enter",
        ctx.env.create_function("enter", on_string_literal_enter)?,
    )?;

    // Next up, `CallExpression`.
    let mut call_expression = ctx.env.create_object()?;
    visitor.set_named_property("CallExpression", &call_expression)?;
    call_expression.set_named_property(
        "enter",
        ctx.env.create_function("enter", on_call_expression_enter)?,
    )?;

    // We'll start by calling the traverser function with our ast and a visitor.
    traverse_node(&ast, None, &visitor)?;

    Ok(new_ast)
}

#[js_function(2)]
fn on_number_literal_enter(ctx: CallContext) -> Result<JsUndefined> {
    let node = ctx.get::<JsObject>(0)?;
    let parent = ctx.get::<JsObject>(1)?;

    // We'll create a new node also named `NumberLiteral` that we will push to
    // the parent context.
    let mut context = parent.get_named_property::<JsObject>("_context")?;
    let mut number_literal = ctx.env.create_object()?;
    number_literal.set_named_property("type", ctx.env.create_string("NumberLiteral")?)?;
    number_literal.set_named_property("value", node.get_named_property::<JsString>("value")?)?;

    context.set_element(context.get_array_length()?, &number_literal)?;

    ctx.env.get_undefined()
}

#[js_function(2)]
fn on_string_literal_enter(ctx: CallContext) -> Result<JsUndefined> {
    let node = ctx.get::<JsObject>(0)?;
    let parent = ctx.get::<JsObject>(1)?;

    // We'll create a new node also named `StringLiteral` that we will push to
    // the parent context.
    let mut context = parent.get_named_property::<JsObject>("_context")?;
    let mut string_literal = ctx.env.create_object()?;
    string_literal.set_named_property("type", ctx.env.create_string("StringLiteral")?)?;
    string_literal.set_named_property("value", node.get_named_property::<JsString>("value")?)?;

    context.set_element(context.get_array_length()?, &string_literal)?;

    ctx.env.get_undefined()
}

#[js_function(2)]
fn on_call_expression_enter(ctx: CallContext) -> Result<JsUndefined> {
    let mut node = ctx.get::<JsObject>(0)?;
    let parent = ctx.get::<JsObject>(1)?;

    // We start creating a new node `CallExpression` with a nested
    // `Identifier`.
    let mut context = parent.get_named_property::<JsObject>("_context")?;
    let mut expression = ctx.env.create_object()?;
    let mut callee = ctx.env.create_object()?;
    let arguments = ctx.env.create_array()?;
    expression.set_named_property("type", ctx.env.create_string("CallExpression")?)?;
    callee.set_named_property("type", ctx.env.create_string("Identifier")?)?;
    callee.set_named_property("name", node.get_named_property::<JsString>("name")?)?;
    expression.set_named_property("callee", &callee)?;
    expression.set_named_property("arguments", &arguments)?;

    // Next we're going to define a new context on the original
    // `CallExpression` node that will reference the `expression`'s arguments
    // so that we can push arguments.
    node.set_named_property("_context", &arguments)?;

    // Then we're going to check if the parent node is a `CallExpression`.
    // If it is not...
    if parent
        .get_named_property::<JsString>("type")?
        .into_utf8()?
        .as_str()?
        != "CallExpression"
    {
        // We're going to wrap our `CallExpression` node with an
        // `ExpressionStatement`. We do this because the top level
        // `CallExpression` in JavaScript are actually statements.
        let mut expression_statement = ctx.env.create_object()?;
        expression_statement
            .set_named_property("type", ctx.env.create_string("ExpressionStatement")?)?;
        expression_statement.set_named_property("expression", &expression)?;
        expression = expression_statement;
    }

    // Last, we push our (possibly wrapped) `CallExpression` to the `parent`'s
    // `context`.
    context.set_element(context.get_array_length()?, &expression)?;

    ctx.env.get_undefined()
}

/**
 * ============================================================================
 *                               ヾ（〃＾∇＾）ﾉ♪
 *                            THE CODE GENERATOR!!!!
 * ============================================================================
 */

/**
 * Now let's move onto our last phase: The Code Generator.
 *
 * Our code generator is going to recursively call itself to print each node in
 * the tree into one giant string.
 */

#[napi]
fn code_generator(ctx: CallContext) -> Result<JsString> {
    let node = ctx.get::<JsObject>(0)?;

    let code = code_generator_node(&node)?;

    ctx.env.create_string_from_std(code)
}

fn code_generator_node(node: &JsObject) -> Result<String> {
    let node_type = node
        .get_named_property::<JsString>("type")?
        .into_utf8()?
        .as_str()?
        .to_string();

    // We'll break things down by the `type` of the `node`.
    let code = match node_type.as_str() {
        // If we have a `Program` node. We will map through each node in the `body`
        // and run them through the code generator and join them with a newline.
        "Program" => {
            let body = node.get_named_property::<JsObject>("body")?;
            let len = body.get_array_length()?;
            let mut vec = Vec::new();
            for index in 0..len {
                vec.push(code_generator_node(&body.get_element::<JsObject>(index)?)?);
            }
            vec.join("\n")
        }

        // For `ExpressionStatement` we'll call the code generator on the nested
        // expression and we'll add a semicolon...
        "ExpressionStatement" => format!(
            "{};",
            code_generator_node(&node.get_named_property::<JsObject>("expression")?)?
        ), // << (...because we like to code the *correct* way)

        // For `CallExpression` we will print the `callee`, add an open
        // parenthesis, we'll map through each node in the `arguments` array and run
        // them through the code generator, joining them with a comma, and then
        // we'll add a closing parenthesis.
        "CallExpression" => {
            let arguments = node.get_named_property::<JsObject>("arguments")?;
            let len = arguments.get_array_length()?;
            let mut vec = Vec::new();
            for index in 0..len {
                vec.push(code_generator_node(
                    &arguments.get_element::<JsObject>(index)?,
                )?);
            }
            format!(
                "{}({})",
                code_generator_node(&node.get_named_property::<JsObject>("callee")?)?,
                vec.join(", ")
            )
        }

        // For `Identifier` we'll just return the `node`'s name.
        "Identifier" => node
            .get_named_property::<JsString>("name")?
            .into_utf8()?
            .as_str()?
            .to_string(),

        // For `NumberLiteral` we'll just return the `node`'s value.
        "NumberLiteral" => node
            .get_named_property::<JsString>("value")?
            .into_utf8()?
            .as_str()?
            .to_string(),

        // For `StringLiteral` we'll add quotations around the `node`'s value.
        "StringLiteral" => format!(
            "\"{}\"",
            node.get_named_property::<JsString>("value")?
                .into_utf8()?
                .as_str()?
                .to_string()
        ),

        // And if we haven't recognized the node, we'll throw an error.
        _ => return Err(Error {
            status: Status::GenericFailure,
            reason: node_type,
        })
    };

    Ok(code)
}

