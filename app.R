library(shiny)
library(shinydashboard)
library(rvest)
library(dplyr)
library(stringr)
library(lubridate)
library(plotly)
library(DT)

scrape_year <- function(url, year) {
  page <- read_html(url)
  rows <- page %>% html_nodes("tr")
  data_list <- list()
  for (row in rows) {
    cells <- row %>% html_nodes("td")
    if (length(cells) >= 9) {
      data_list[[length(data_list) + 1]] <- data.frame(
        Date = html_text(cells[1], trim = TRUE),
        LastName = html_text(cells[2], trim = TRUE),
        FirstName = html_text(cells[3], trim = TRUE),
        Race = html_text(cells[4], trim = TRUE),
        Sex = html_text(cells[5], trim = TRUE),
        Age = html_text(cells[6], trim = TRUE),
        Method = html_text(cells[7], trim = TRUE),
        District = html_text(cells[8], trim = TRUE),
        Location = html_text(cells[9], trim = TRUE),
        Year = year
      )
    }
  }
  if (length(data_list) > 0) return(do.call(rbind, data_list))
  return(data.frame())
}

all_data <- bind_rows(
  scrape_year("https://homicides.news.baltimoresun.com/?page=2023", "2023"),
  scrape_year("https://homicides.news.baltimoresun.com/?page=2024", "2024"),
  scrape_year("https://homicides.news.baltimoresun.com/?page=2025", "2025")
) %>%
  filter(Date != "", Date != "Date") %>%
  mutate(
    Age_num = as.numeric(Age),
    Date_parsed = mdy(Date),
    Month = month(Date_parsed, label = TRUE)
  )

ui <- dashboardPage(
  dashboardHeader(title = "Baltimore Homicide Dashboard"),
  dashboardSidebar(
    sidebarMenu(
      menuItem("Dashboard", tabName = "dashboard", icon = icon("dashboard")),
      menuItem("Data Table", tabName = "table", icon = icon("table"))
    ),
    hr(),
    h4("Filters", style = "padding-left: 15px;"),
    selectInput("year_filter", "Year:", choices = c("All" = "all", "2023", "2024", "2025"), selected = "all"),
    sliderInput("age_filter", "Age:", min = 0, max = 110, value = c(0, 110)),
    checkboxGroupInput("method_filter", "Method:", choices = c("All"), selected = "All"),
    checkboxGroupInput("sex_filter", "Sex:", choices = c("All"), selected = "All")
  ),
  dashboardBody(
    tabItems(
      tabItem(tabName = "dashboard",
        fluidRow(
          valueBoxOutput("total_homicides"),
          valueBoxOutput("avg_age"),
          valueBoxOutput("most_common_method")
        ),
        fluidRow(
          box(title = "Homicides by Month", plotlyOutput("monthly_plot"), width = 6),
          box(title = "Age Distribution", plotlyOutput("age_plot"), width = 6)
        ),
        fluidRow(
          box(title = "Homicides by Method", plotlyOutput("method_plot"), width = 6),
          box(title = "Homicides by District", plotlyOutput("district_plot"), width = 6)
        )
      ),
      tabItem(tabName = "table",
        box(title = "Homicide Records", DTOutput("data_table"), width = 12)
      )
    )
  )
)

server <- function(input, output, session) {
  observe({
    methods <- unique(all_data$Method)
    methods <- methods[methods != "" & !is.na(methods)]
    updateCheckboxGroupInput(session, "method_filter", choices = c("All", sort(methods)), selected = "All")
    sexes <- unique(all_data$Sex)
    sexes <- sexes[sexes != "" & !is.na(sexes)]
    updateCheckboxGroupInput(session, "sex_filter", choices = c("All", sort(sexes)), selected = "All")
  })
  filtered_data <- reactive({
    data <- all_data
    if (input$year_filter != "all") data <- data %>% filter(Year == input$year_filter)
    data <- data %>% filter(!is.na(Age_num)) %>% filter(Age_num >= input$age_filter[1] & Age_num <= input$age_filter[2])
    if (!"All" %in% input$method_filter && length(input$method_filter) > 0) data <- data %>% filter(Method %in% input$method_filter)
    if (!"All" %in% input$sex_filter && length(input$sex_filter) > 0) data <- data %>% filter(Sex %in% input$sex_filter)
    data
  })
  output$total_homicides <- renderValueBox({ valueBox(nrow(filtered_data()), "Total Homicides", icon = icon("exclamation-triangle"), color = "red") })
  output$avg_age <- renderValueBox({ valueBox(round(mean(filtered_data()$Age_num, na.rm = TRUE), 1), "Avg Age", icon = icon("user"), color = "blue") })
  output$most_common_method <- renderValueBox({
    method_counts <- filtered_data() %>% filter(Method != "") %>% count(Method, sort = TRUE)
    valueBox(if(nrow(method_counts) > 0) method_counts$Method[1] else "N/A", "Most Common Method", icon = icon("list"), color = "yellow")
  })
  output$monthly_plot <- renderPlotly({
    data <- filtered_data() %>% filter(!is.na(Month)) %>% count(Month)
    if (nrow(data) == 0) return(plotly_empty())
    plot_ly(data, x = ~Month, y = ~n, type = "bar", marker = list(color = "#3498db")) %>% layout(xaxis = list(title = "Month"), yaxis = list(title = "Count"))
  })
  output$age_plot <- renderPlotly({
    data <- filtered_data() %>% filter(!is.na(Age_num))
    if (nrow(data) == 0) return(plotly_empty())
    plot_ly(data, x = ~Age_num, type = "histogram", marker = list(color = "#e74c3c")) %>% layout(xaxis = list(title = "Age"))
  })
  output$method_plot <- renderPlotly({
    data <- filtered_data() %>% filter(Method != "") %>% count(Method, sort = TRUE) %>% head(10)
    if (nrow(data) == 0) return(plotly_empty())
    plot_ly(data, x = ~Method, y = ~n, type = "bar", marker = list(color = "#2ecc71")) %>% layout(xaxis = list(title = "Method"))
  })
  output$district_plot <- renderPlotly({
    data <- filtered_data() %>% filter(District != "") %>% count(District, sort = TRUE)
    if (nrow(data) == 0) return(plotly_empty())
    plot_ly(data, labels = ~District, values = ~n, type = "pie")
  })
  output$data_table <- renderDT({
    filtered_data() %>% select(Date, FirstName, LastName, Age, Sex, Race, Method, District, Location, Year) %>% datatable(options = list(pageLength = 25, scrollX = TRUE))
  })
}
shinyApp(ui = ui, server = server)
