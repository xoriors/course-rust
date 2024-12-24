# For the teachers to follow

## Course setup

1. At the beginning of each course, make sure the material is ready in [docs](.) and [assignments](../assignments)
2. [assignments](../assignments) structure (usually this exists already. Make sure it's valid and change as needed)
  - A folder for each chapter, like `Chapter_1` with the following structure:
    - `requirements` folder: Containing a file called `Chapter_N_assignment.md` with the requirements and any other needed resources
    - `solutions` folder: Will contain folders with the student's GitHub account ID, each containing submissions from each student. Each will have an associated PR
3. Create a new project called `course-<year>`, where `<year>` is the course's year, mostly the current year
  - Create GitHub issues for each chapter and assignment, adding all needed docs with relevant links to any resources. All issue descriptions should have a link to [students.md]
  - For the final project, create an assignment issue titled `Final project` and have the requirements in 

## Into the course, for each week (example for Chapter 1)

1. Mark the status `In Progress` for the current chapter (let's call it Chapter 1). This is when students begin reading it
2. Resolve any comments and questions in the chapter's issue
3. After the chapter is finished, this is generally after the weekly call for the last week's chapter, mark the issue as `Done`
4. Inform students about the chapter assignment and set the status of the issue to `In Progress`
5. Resolve any comments and questions in the assignment's issue
7. Review any PRs to the assignment issue and merge them when completed
8. After all assignments were addressed mark the assignment issue as `Done`

## Final project

Follow similar steps to `Into the course, for each week (example for Chapter 1)` just ignoring the chapter part.

## After the course

- After all issues and discussions have been resolved, create a new branch called `course-<year>`, where `<year>` is the course's year
- Close the project for this year from project settings
