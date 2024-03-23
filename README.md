# Group Numba One (This is our teamname)
# Derek Warner (derekw3), Chengxun Ren (cren8), Haozhe Chen (haozhe6), Aaryan Singh Gusain (agusain2)

Our project will be a rust based stock analysis program with a CLI interface taking in stock information such as a ticker and using API's to fetch the correct data and predict the stock and give a basic analysis on if the stock should be bought or sold, all of which will use various ML methods.

Main Goals:
- CLI user interface for input/output
- Predict stocks based on past data
- Visualize stock data
- Basic suggestions on if stock should be bought/sold

Technical Specifications:
CLI Interface: the cli interface will run in a basic Linux terminal and we will process cli arguments given by the user using the clap crate.

Stock Data Retrieval: API use to fetch stock data for requested stocks and for how long, most likely this will be done with the Yahoo! Finance API crate.

ML Methods: We will perform ML methods both mathematically and through the use of various ML crates in the cargo environment, but we will not be using direct crates which do the stock analysis for us.

Visualize Stock Data: We will use either a plotting crate such as plotters or perhaps an interesting CLI plotting crate like textplots (the aesthetic is cool)

Basic Suggestions: The program will use the ML factors and see if, based on the data, it is worthwile to buy or sell a certain stock based on if it will grow or fall.

Checkpoints:
1st - For the first checkpoint we would like to finish the setup of the project, the CLI interface and the data feching, all being tested properly.

2nd - For the second checkpoint we would like to get the rest of the base features implemented and begin to refine them.

Possible Challenges:
Based on the relatively new ecosystem of Rust ML we may need to program a lot of the specific mathematical features ourselves. We also may need to write some complex tests to test these functions.
