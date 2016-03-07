defmodule DNA do
  @nucleotides [?A, ?C, ?G, ?T]

  @doc """
  Counts individual nucleotides in a DNA strand.

  ## Examples

  iex> DNA.count('AATAA', ?A)
  4

  iex> DNA.count('AATAA', ?T)
  1
  """
  @spec count([char], char) :: non_neg_integer
  def count(strand, nucleotide) do
    validate_nucleotide(nucleotide)
    validate_strand(strand)
    
    strand
    |> Enum.filter(&(&1 == nucleotide))
    |> Enum.count
  end


  @doc """
  Returns a summary of counts by nucleotide.

  ## Examples

  iex> DNA.histogram('AATAA')
  %{?A => 4, ?T => 1, ?C => 0, ?G => 0}
  """
  @spec histogram([char]) :: Map
  def histogram(strand) do
    validate_strand(strand)

    strand
    |> Enum.reduce(empty_nucelotide_count, &count_strand/2)
  end

  defp count_strand(n, acc) do
    Map.update!(acc, n, &(&1 + 1))
  end

  defp empty_nucelotide_count do
    @nucleotides |> Enum.into(%{}, &({&1, 0}))
  end

  defp validate_strand(strand) do
    strand
    |> Enum.each(&validate_nucleotide/1)
  end

  defp validate_nucleotide(nucleotide) do
    unless Enum.any?(@nucleotides, fn(x) -> x == nucleotide end) do
      raise ArgumentError
    end
  end
end