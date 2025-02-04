# PineScript to **PACE** migration guide

## ChatGPT

This is a work in progress. GPT may not be able to translate all PineScript code to Pace and give wrong code. Use this with caution.

> Note: GPT-4 has a maximum context size of 8192 tokens. If you exceed this limit, GPT may make mistakes and halucinate.
>
> Use [tokenizer counter](https://platform.openai.com/tokenizer), if you are not sure.

You can use GPT-4 to directly translate PineScript code to Pace.

1. Go to [GPT-4 playground](https://platform.openai.com/playground?mode=chat&model=gpt-4)

2. Select mode `Chat`

3. Select model `gpt-4`

4. Set the temperature to `0`

5. Set maximum length to `2028` (max)

6. Copy paste [system prompt](https://raw.githubusercontent.com/nersent/pace/main/docs/gpt/system_prompt.md) (left panel)

7. Copy paste [user prompt](https://raw.githubusercontent.com/nersent/pace/main/docs/gpt/user_prompt.md) (right panel)

8. Click submit button
