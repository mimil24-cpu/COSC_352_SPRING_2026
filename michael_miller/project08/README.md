# Project 8 - Part 2: Baltimore City Open Data Analysis
**COSC 352 - Michael Miller**

## Dataset 1: BPD Part 1 Crime Data
- **Source URL**: https://data.baltimorecity.gov/datasets/bpd-part-1-crime-data
- **Description**: Baltimore Police Department Part 1 crime incidents reported across the city, including location, crime type, weapon used, and police district.
- **Key Columns Used**: `Neighborhood`, `Description` (crime type), `District`, `CrimeDate`

## Dataset 2: 311 Customer Service Requests
- **Source URL**: https://data.baltimorecity.gov/datasets/311-customer-service-requests
- **Description**: All 311 service requests submitted by Baltimore City residents, including service type, status, and location.
- **Key Columns Used**: `Neighborhood`, `SRType`, `SRStatus`, `SRCreatedDate`

## Research Question
Do Baltimore City neighborhoods with higher volumes of 311 service requests also experience higher crime counts? If so, this would suggest a link between neighborhood maintenance/quality-of-life issues and crime rates.

## Answer
After joining both datasets on `Neighborhood` and computing a Pearson correlation coefficient between 311 call volume and crime count per neighborhood, the analysis found a **moderate positive correlation (r ≈ 0.62)** across the 30 matched neighborhoods. Neighborhoods such as Sandtown-Winchester, Belair-Edison, and Park Heights appeared in the top tier for both metrics, with 311 call volumes above 180 per neighborhood and crime counts above 110. This is consistent with the theory that neighborhoods under greater infrastructural and social stress generate both more service requests and more crime incidents. The correlation does not imply causation -- both variables are likely driven by underlying socioeconomic factors.

## Limitations
1. The dataset used in this analysis is a sample (3,000 crime records, 5,000 311 calls) and may not fully represent the complete distribution of incidents across all years.
2. Neighborhood name matching is case-sensitive and exact; slight variations in how names are recorded across datasets could cause some neighborhoods to be excluded from the join.
3. The analysis does not account for population density per neighborhood, which would be necessary to compute per-capita rates.

## How to Run
```bash
# Profile Dataset 1
cargo run --bin csvprof -- data/bpd_crime.csv > reports/bpd_crime_profile.txt

# Profile Dataset 2
cargo run --bin csvprof -- data/311_calls.csv > reports/311_calls_profile.txt

# Run Part 2 correlation analysis
cargo run --bin correlate
```

## File Structure
```
project08/
  data/
    bpd_crime.csv        # BPD Part 1 Crime Data (3000 records)
    311_calls.csv        # 311 Customer Service Requests (5000 records)
  reports/
    bpd_crime_profile.txt  # csvprof output for crime data
    311_calls_profile.txt  # csvprof output for 311 data
  src/
    lib.rs               # Part 1: ColumnAnalyzer trait, StatsAnalyzer, stream_csv, CsvProfError
    main.rs              # Part 1: csvprof binary CLI
    correlate.rs         # Part 2: joins both datasets, computes Pearson r
  Cargo.toml
  README.md
```
