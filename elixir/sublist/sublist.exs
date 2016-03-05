defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare(a, b) do
    cond do
      a === b -> :equal
      sublist?(a, b) -> :sublist
      sublist?(b, a) -> :superlist
      true -> :unequal
    end
  end

  defp sublist?(a,b) when a === [], do: true
  defp sublist?(a,b) do
    b
    |> Stream.chunk(Enum.count(a), 1)
    |> Stream.drop_while(&(a !== &1))
    |> Enum.any?
  end

end
