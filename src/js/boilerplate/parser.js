const synonyms = Object.freeze({
    'for': 'each',
});

const buzz = Object.freeze([
    'the',
]);

const parseInput = (input) => {
    input = input.toUpperCase();

    // splits into tokens
    const tokens = input.split(' ');

    // buzz tokens
    for (let i = 0; i < tokens.length; i++) {
        const token = tokens[i];
        if (buzz.includes(token)) {
            tokens.splice(i, 1);
            i -= 1;
        }
    }

    // for each token, replace it with its top_level synonym if it exists
    // any replacements made here cannot be objects
    for (let i = 0; i < tokens.length; i++) {
        const token = tokens[i];
        if (token in synonyms) {
            tokens[i] = synonyms[token];
        }
    }

    // TODO: handle <direction> or <walk> <direction>

    // find objects
    // OBJECT in a SYNTAX always refers to an OBJECT, and never a ROOM?
    // need to build a synonym object for OBJECTS, where the value is a list of objects
    // take note of any replacements, look for adjectives nearby

    // need to re-think syntax
    // first word is always the verb
    // every other non-OBJECT word is a preposition
    // there can be zero overlap between OBJECT synonyms and prepositions
    // there can be zero overlap between BUZZ and object synonyms
    // there can be zero overlap between BUZZ and prepositions
    // there can be zero overlap between BUZZ and verbs

    // 0. split into sentences, handle each in turn, pause if fatal
    // 1. handle <direction> or <walk> <direction>
    // 2. Use https://github.com/spencermountain/compromise to identify parts of speech.
    // 3. if first word is not a verb, command is invalid
    // 4. if more than two noun phrases, command is invalid. I'm assuming that compromise has something analagous to 'noun phrase'
    // 5. first noun phrase is always the PRSO, second noun phrase is always the PRSI
    // 6. find potential syntaxes
    //   0. replace noun phrases with OBJECT
    //   1. buzz everything not important
    //   2. apply synonyms
    //   3. look for a matching syntax
    // 7. if no syntax found, command is invalid
    // 8. handle noun phrases
    //   - handle 'all' and 'everything'
    //   - don't worry about 'except' for now
    //   - use LOC
    // 9. re-asses syntaxes (make sure OBJECT restrictions are fulfilled)

    const command = tokens[0];
    const args = tokens.slice(1);
    return { command, args };
}