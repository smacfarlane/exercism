defmodule DNA do
  @doc """
  Transcribes a character list representing DNA nucleotides to RNA

  ## Examples

  iex> DNA.to_rna('ACTG')
  'UGAC'
  """
  @spec to_rna([char]) :: [char]
  def to_rna(dna) do
    Enum.map(dna, &dna_to_rna/1)
  end

  defp dna_to_rna nucleotide do
    case nucleotide do
      ?G -> ?C
      ?T -> ?A
      ?C -> ?G
      ?A -> ?U
    end
  end

end
