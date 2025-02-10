# For the teachers to follow

## Course setup

1. Make sure you're on the chat or other communication channels with the rest of the team
2. At the beginning of each course, make sure the material is ready in [/docs](.) and [/assignments](../assignments)
3. [/assignments](../assignments) structure (usually this exists already. Make sure it's valid and change as needed):  
   A folder for each chapters group, like `Chapter_1-3` with the following structure:
     - `requirements` folder: Containing a file called `Chapter_N-M_assignment.md` with the requirements and any other needed resources
     - `solutions` folder: This folder will contain folders with each student's GitHub account ID, each containing submissions from each student, each with an associated PR.  
       Make sure this folder is empty so it will contain the solutions from this session
4. Create a new project called `course-<year>`, where `<year>` is the course's year, mostly the current year
  - Create GitHub issues for each chapter and assignment, adding all needed docs with relevant links to any resources. All issue descriptions should have a link to [students.md]
  - For the final project, create an assignment issue titled `Final project` and have the requirements in 

## Into the course, for each week

1. Create a GH issue for the theory part for the given chapters for the next week session, like `Course 4` and include the table of contents of the chapters
2. Create a GH issue for the home assignment for the given chapters for the last week session, like `Course 1-3` and include the requirements and any needed files in there
3. Have the weekly call of about 2h to discuss:
  - Review last week's assignment (this corresponds to the chapter from 2 weeks back); let's call it `Chapter N-2: Assignment`. After this is done, mark the issue as `Done`
  - Discuss this week's chapters, `Chapter 1-3` clarifying any questions
  - New home assignment for this week's chapters. Show them the GH issue for it and instruct them to create PR in there and ask their questions
  - Intro to ds and show the GH issue for it, instructing the students to add their questions in there
  - live coding or present building part of the course main app related to last week's chapters knowledge
4. 
5. 
6. After this is done, mark the issue as `Done`
7. Give the assignment for last week's chapter (this is called this week's assignment), discuss any questions, and set the issue's status to `In Progress`
8. Move to this week's chapter, `Chapter 3: <Title>`, and do a short intro. Mark the status `In Progress` for the current chapter.
9. This is where the call ends
10. Over the week, resolve any comments and questions in the week's chapter and assignment issues and on Slack
11. Record everything and save videos in `videos/Chapter_N`

## Final project

Follow similar assignment steps to `Into the course, for each week`, ignoring the chapter steps.

## After the course

- After all issues and discussions have been resolved, create a new branch called `course-<year>`, where `<year>` is the course's year
- Close the project for this year from project settings
