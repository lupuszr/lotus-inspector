#!/usr/bin/env zx
// import 'zx/globals';

let command = process.argv[3];

// let command = await question("Choose command: ", {
//     choices: ["db:migrate", "db:undo", "help"]
// });


switch(command) {
    case "db:migrate": {
        $`diesel migration run`
        break;
    }
    case "db:undo": {
        $`diesel migration redo`
        break;
    }
    case "help": {
        console.log(`
        "db:migrate" - migrate db (for more info search for diesel)
        "db:undo" - undo migration
        "help" - show this help
        `)
    }
}



