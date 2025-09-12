library(shiny)
ui <- fluidPage(
  titlePanel("My Simple Shiny App"),

  sidebarLayout(
    sidebarPanel(
      # No inputs for this very simple app
    ),

    mainPanel(
      h3("Welcome to Shiny!"),
      p("This is a basic example of a Shiny application.")
    )
  )
)

server <- function(input, output) {
  # No server-side logic for this very simple app
}

shinyApp(ui = ui, server = server)
