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

func main() {
	app := fiber.New()
	env, port := "", ""

	err := godotenv.Load(".env")
	if err != nil {
		port = "3001"
		env = "dev"
		print("Error loading .env file")
	} else {
		port = os.Getenv("PORT")
		env = os.Getenv("ENV")
	}

	fmt.Printf("%s, %s", port, env)

	app.Get("/go", func(c *fiber.Ctx) error {
		msg := fmt.Sprintf("Go Service on env : %s -- version 0.0.1 !", env)
		return c.JSON(Response{Message: msg})
	})

	app.Listen(fmt.Sprintf(":%s", port))
}
