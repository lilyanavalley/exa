
# Product - Common product names
-product = Experiment A
-product-abbreviation = EXA

# Product Package - Identifies what system this game was packaged for.
product-package =
    { $target -> 
        [wasm] WASM
        [macos] MacOS
        [linux] Linux
        [windows] Windows
        *[unpackaged] portable
    }

# CoreMenu - Menu interface terms.
coremenu = Menu
    .button-exit = Exit
    .button-fosscredits = Credits
    .button-settings = Settings
    .button-game-load = Load Game
    .button-game-new = New Game
    .button-game-resume = Resume Game
    .copyright-notice = ©️ { $authors }, 2024 under GPL-3
    .version = v{ $version }
    .loading = ··· LOADING ···
    .saves-title = Saves
    .saves-folder = Open Saves Folder

# Dialog testing character names
char-player = Player
char-game = Game

# Dialog testing lines
d-test = 
    Fluent provides internationalization layering and mechanisms to create natural-sounding strings for apps like 
    {-product-abbreviation}. 
    We use Fluent with Serde, RON and a custom-built 'Dialog' component to compose conversations between characters!

d-test-response = 
    Dialog works akin to RPGs and may take diverging/converging paths with an expandable, non-linear *storyline* 
    subsystem. Lastly, texts fed to the Dialog component may be formatted in **Markdown** (GFM) for emphasis.

    # Headings
    are also an option for attention-grabbing titles or subheadings.

    > There are blockquotes

    `code blocks`

    ```
        with multiline
    ```

    - bullets for points!

    1. Numbered lists...
    2. ...

    \[x\] or checkboxes...

    \[A link for prosperity\](https://www.thefreedictionary.com/prosperity), if you so choose.

    ![Image](data/)

d-test-choice-a = Choice A
d-test-choice-a-outcometext = Expand
d-test-choice-b = Choice B
d-test-choice-b-outcometext = Exit
d-test-choice-c = Choice C
d-test-choice-c-outcometext = Repeat
