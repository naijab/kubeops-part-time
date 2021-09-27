package main

import (
	"fmt"
	"os"

	"github.com/gofiber/fiber/v2"
	"github.com/joho/godotenv"
)

type Response struct {
	Message string `json:"message"`
}

func getenv(key, fallback string) string {
	value := os.Getenv(key)
	if len(value) == 0 {
		return fallback
	}
	return value
}

func main() {
	app := fiber.New()
	env, port := "dev", "3001"

	err := godotenv.Load(".env")
	if err != nil {
		print("Error loading .env file")
	}

	env = getenv("ENV", "dev")
	port = getenv("PORT", "3001")

	app.Get("/go", func(c *fiber.Ctx) error {
		msg := fmt.Sprintf("Go Service on env : %s -- version 0.0.3 !", env)
		return c.JSON(Response{Message: msg})
	})

	app.Listen(fmt.Sprintf(":%s", port))
}
