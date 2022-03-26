# annoying_journal

A journal that pops up and steals keyboard focus every 37 minutes,
to force you to write an entry.

<video autoplay loop muted>
<source src="videos/002-demo.webm" type="video/webm">
</video>

## Output

Text and timestamps of journal entries are saved in the directory `annoying_journal`,
in [JSONL format](https://jsonlines.org/).

## CLI parameters

- `--interval-secs <u64>` - Change the pop-up interval (default is 2,225)
- `--prompt <String>` - Change the prompt that shows above the editor.
