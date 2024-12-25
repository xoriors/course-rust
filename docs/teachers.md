# For the teachers to follow

## Course setup

1. Join [Slack channel](https://xorio.slack.com/archives/C0869LC717B). Create an account on Slack if needed. You can do it with your Google account
2. At the beginning of each course, make sure the material is ready in [docs](.) and [assignments](../assignments)
3. [assignments](../assignments) structure (usually this exists already. Make sure it's valid and change as needed):  
   A folder for each chapters group, like `Chapter_1-3` with the following structure:
     - `requirements` folder: Containing a file called `Chapter_N-M_assignment.md` with the requirements and any other needed resources
     - `solutions` folder: This folder will contain folders with each student's GitHub account ID, each containing submissions from each student, each with an associated PR.  
       Make sure this folder is empty so it will contain the solutions from this session
4. Create a new project called `course-<year>`, where `<year>` is the course's year, mostly the current year
  - Create GitHub issues for each chapter and assignment, adding all needed docs with relevant links to any resources. All issue descriptions should have a link to [students.md]
  - For the final project, create an assignment issue titled `Final project` and have the requirements in 

## Into the course, for each week

1. Have the weekly call of about 2h to discuss last week's chapter and assignment and intro to this week's chapter and assignment
2. Review last week's assignment (this corresponds to the chapter from 2 weeks); let's call it `Chapter 1: Assignment`. After this is done, mark the issue as `Done`
3. Discuss last week's chapter, `Chapter 2: <Title>` clarifying any questions
4. After this is done, mark the issue as `Done`
5. Give the assignment for last week's chapter (this is called this week's assignment), discuss any questions, and set the issue's status to `In Progress`
6. Move to this week's chapter, `Chapter 3: <Title>`, and do a short intro. Mark the status `In Progress` for the current chapter.
7. This is where the call ends
8. Over the week, resolve any comments and questions in the week's chapter and assignment issues and on Slack
9. Record everything and save videos in `videos/Chapter_N`

## Final project

Follow similar assignment steps to `Into the course, for each week`, ignoring the chapter steps.

## After the course

- After all issues and discussions have been resolved, create a new branch called `course-<year>`, where `<year>` is the course's year
- Close the project for this year from project settings
