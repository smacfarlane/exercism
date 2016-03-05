defmodule Anagram do
  @doc """
  Returns all candidates that are anagrams of, but not equal to, 'base'.
  """
  @spec match(String.t, [String.t]) :: [String.t]
  def match(base, candidates) do
    ordered_base = order(base)
    candidates
    |> Enum.reject(&same?(base, &1))
    |> Enum.filter(&same?(order(&1),ordered_base))
  end

  defp order(string) do
    string
    |> String.downcase
    |> String.graphemes
    |> Enum.sort
    |> Enum.join
  end

  defp same?(a, b) do
    String.downcase(a) === String.downcase(b)
  end
end
