defmodule PactElixir.RustPactMockServerFacade do
  @moduledoc """
  Adapter for the wrapped rust [pact mock server](https://github.com/pact-foundation/pact-reference).
  Functions in this file are replaced by Rustler with their Rust calling
  counterpart. See native/pactmockserver/src/lib.rs for the concrete Rust
  implementation.
  This file is excluded from the coverage tool.
  """

  use Rustler, otp_app: :pact_elixir, crate: "pactmockserver"

  # The return type is: {:ok, port} or {:error, something}
  def create_mock_server(_pact_json, _port), do: throw(:nif_not_loaded)
  def mock_server_mismatches(_port), do: throw(:nif_not_loaded)

  # @spec mock_server_matched(number) :: {:ok, boolean}
  def mock_server_matched(_port), do: throw(:nif_not_loaded)
  def write_pact_file(_port, _dir_path), do: throw(:nif_not_loaded)
  def cleanup_mock_server(_port), do: throw(:nif_not_loaded)
end
