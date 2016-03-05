defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "HORSE" => "1H1O1R1S1E"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "1H1O1R1S1E" => "HORSE"
  """
  @spec encode(string) :: String.t
  def encode(string) do
    string
    |> String.graphemes
    |> Enum.chunk_by(&(&1))
    |> Enum.flat_map(&([length(&1)] ++ [hd(&1)]))
    |> Enum.join
  end

  @spec decode(string) :: String.t
  def decode(string) do
    Regex.scan(~r/(\d+)(.)/, string)
    |> Enum.map(&expand(&1))
    |> Enum.join
  end

  defp expand([_, count, letter]) do
    String.duplicate(letter, String.to_integer(count))
  end
end
