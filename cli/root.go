package cli

import (
	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/cli/server"
	"gitlab.com/bloom42/bloom/cli/version"
)

func init() {
	rootCmd.AddCommand(server.ServerCmd)
	rootCmd.AddCommand(version.VersionCmd)
}

var rootCmd = &cobra.Command{
	Use:   "bloom",
	Short: "Bloom",
	Long:  "Bloom: A safe palce for all your data. Visit https://bloom.sh for more information.",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func Execute() error {
	return rootCmd.Execute()
}
