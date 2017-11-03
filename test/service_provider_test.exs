defmodule PactElixir.ServiceProviderTest do
  use ExUnit.Case
  alias PactElixir.ServiceProvider

  test "default constructor values" do
    provider = ServiceProvider.new()

    assert 0 == provider.port
  end

  test "constructor parameters" do
    provider = ServiceProvider.new(port: 52342)

    assert 52342 == provider.port
  end
end
