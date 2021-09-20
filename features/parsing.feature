Feature: library should give opportunity to parse user-friendly string to chrono date and vice versa

  Scenario Outline: User-friendly string to chrono date
    Given user-friendly <string>
    And implemented parser
    And relative date set as "2021-09-20T00:00:00Z"
    When friendly_parse function is called
    Then function should return expected <date>
    Examples:
      | string        | date                 |
      | today         | 2021-09-20T00:00:00Z |
      | tomorrow      | 2021-09-21T00:00:00Z |
      | yesterday     | 2021-09-19T00:00:00Z |
      | other-defined | 2002-02-22T00:00:00Z |
