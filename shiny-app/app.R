library(shiny)
ui <- fluidPage(
  titlePanel("My Interactive Shiny App"),

  sidebarLayout(
    sidebarPanel(
      h4("Input Control"),
      sliderInput(
        inputId = "num",
        label = "Choose a number:",
        min = 1,
        max = 100,
        value = 50
      )
    ),

    mainPanel(
      h3("Welcome to Shiny!"),
      p(
        "This is an example of a Shiny application with a simple input and output."
      ),
      br(),
      h4("Your selection:"),
      textOutput("selected_num_output"),
      br(), # Added a line break for better spacing before the plot
      h4("Interactive Plot:"), # Added a heading for the plot
      plotOutput("distPlot") # Added plotOutput
    )
  )
)

server <- function(input, output) {
  output$selected_num_output <- renderText({
    paste("You have selected the number:", input$num)
  })

  # Added a renderPlot to display a histogram
  output$distPlot <- renderPlot({
    # Generate 'input$num' random normal numbers
    hist_data <- rnorm(input$num)

    # Draw the histogram with a dynamic title
    hist(
      hist_data,
      main = paste("Histogram of", input$num, "Random Numbers"),
      xlab = "Value",
      col = 'steelblue',
      border = 'white'
    )
  })
}

shinyApp(ui = ui, server = server)
