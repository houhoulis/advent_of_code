defmodule Sonar do
  def deeper_measurements(input) do
    {:ok, measurements} = File.read(input)

    measurements
    |> String.split()
    |> Enum.chunk_every(2, 1, :discard)
    |> IO.inspect()
    |> Enum.count(fn [x, y] -> String.to_integer(y) > String.to_integer(x) end)
  end

  def deeper_windows(input) do
    {:ok, measurements} = File.read(input)

    measurements
    |> String.split()
    |> Enum.chunk_every(3, 1, :discard)
    |> IO.inspect()
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.count(fn [x, y] -> intsum(y) > intsum(x) end)
  end

  defp intsum(list) do
    list
    |> Enum.map(&String.to_integer(&1))
    |> Enum.sum()
  end
end
