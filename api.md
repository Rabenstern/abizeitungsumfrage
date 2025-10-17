# API


| API endpoint | method | behaviour |
|---|---|---|
| `/api` | GET | hello message |
| `/api` | POST | return 200 |
| `/api/authed` | GET | is auth valid |
| `/api/students` | GET | students list |
| `/api/students?email=` | GET | student with email |
| `/api/student/{id}` | GET | student info |
| `/api/teachers` | GET | teachers list |
| `/api/teacher/{id}` | GET | teacher info |
| `/api/qustions` | GET | questions list |
| `/api/qustion/{id}` | GET | question info |
| `/api/answers` | GET | answers list |
| `/api/answers?sid=` | GET | answers with sid |
| `/api/answers?qid=` | GET | answers with qid |
| `/api/answers?sid=&qid=` | GET | answer with sid and qid |
| `/api/answer/{id}` | GET | answer info |
| `/api/answer` | POST | submit answer |

