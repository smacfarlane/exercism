defmodule Words do
  @doc """
  Count the number of words in the sentence.

  Words are compared case-insensitively.
  """
  @spec count(String.t) :: map()
  def count(sentence) do
    sentence
    |> to_words
    |> count_words
  end

  defp to_words sentence do
    sentence
    |> String.split(~r/(\s+|_)/u)
    |> Enum.map(&sanitize/1)
    |> Enum.reject(fn x -> x == "" end)
  end

  defp count_words words do
    Enum.reduce(words, %{}, &increment_value/2)
  end

  defp increment_value(word, acc) do
    Map.update(acc, word, 1, fn x -> x + 1 end)
  end

  def sanitize(word) do
    String.replace(word, ~r/\W+$/u, "")
    |> String.downcase
  end
end
