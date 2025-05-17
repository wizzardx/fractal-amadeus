<!-- 
Suggested repo path: /docs/whitepapers/double_loop_tdd.md
-->

# Double Loop TDD: Outside-In Test-Driven Development for Modern Software Systems

## Abstract

Outside-In Test-Driven Development, also known as Double Loop TDD, represents a sophisticated evolution of traditional TDD practices that addresses the complexities of modern software development. This methodology guides developers to begin with high-level, user-focused tests that drive the implementation of features across multiple system layers. By working from the outside in, teams ensure they're building software that not only functions correctly but also delivers genuine user value.

As applications increasingly integrate complex user interfaces, third-party APIs, microservices, and AI-driven components, Double Loop TDD provides a structured approach to managing this complexity while maintaining focus on user needs. This methodology is particularly valuable for teams working on web applications, service-oriented architectures, and systems with emergent behavior, where traditional bottom-up development approaches can lead to well-tested components that fail to integrate effectively or miss critical user requirements.

## Introduction

### The Challenge of Modern Software Integration

Modern software rarely exists in isolation. Today's applications must seamlessly integrate across numerous layers—from intuitive user interfaces through business logic to infrastructure components and external services. This integration complexity presents substantial challenges:

1. **Cross-cutting concerns**: Features often span multiple architectural layers
2. **Emergent behavior**: System behavior emerges from the interaction of components
3. **External dependencies**: Integration with third-party APIs introduces uncertainty
4. **User expectations**: End users expect intuitive, reliable experiences regardless of underlying complexity

Traditional development approaches often struggle with these challenges. Bottom-up development can produce well-tested components that fail to meet actual user needs. Feature-driven development without proper test discipline can result in brittle implementations that are difficult to maintain or extend.

### Harry Percival and the Testing Goat

Outside-In TDD gained significant attention through Harry J.W. Percival's influential book, *Test-Driven Development with Python*, commonly known as *Obey the Testing Goat*. Percival effectively synthesized and articulated the double-loop approach, drawing on established testing practices while adapting them for modern web application development.

The "Testing Goat" became a memorable metaphor for the disciplined, persistent practice of test-driven development. As Percival explains, the Testing Goat represents the voice urging developers to write tests first, resist shortcuts, and maintain discipline throughout the development process.

### The Core Philosophy: Start With What the User Wants to Do

At its foundation, Outside-In TDD embodies a simple but powerful principle: *start with what the user wants to do*. Rather than beginning with implementation details or technical architecture, developers first capture user requirements as executable tests. These high-level tests serve as a contract, defining acceptance criteria for features from the user's perspective.

This approach ensures that development efforts remain focused on delivering actual value rather than implementing technically impressive solutions that miss the mark for users. By working from the outside in, the methodology naturally encourages developers to think in terms of interfaces, contracts, and user experiences before diving into implementation details.

## Core Methodology

Outside-In TDD operates on two distinct but interconnected feedback loops—hence the name "Double Loop TDD." These loops work together to guide development from high-level user requirements down to the implementation details that satisfy those requirements.

### The Outer Loop: Functional Tests

The outer loop begins with functional tests that represent complete user journeys through the system. These tests:

1. **Focus on user perspective**: Written in terms of user behaviors and expectations
2. **Cross system boundaries**: Typically span multiple components or layers
3. **Provide acceptance criteria**: Define when a feature is "done" from the user's perspective
4. **Remain implementation-agnostic**: Concentrate on what the system should do, not how

Functional tests typically interact with the system the same way a user would—through web browsers (using tools like Selenium), API endpoints, command-line interfaces, or other user-facing interfaces. When first written, these tests will fail immediately since no implementation exists to satisfy them.

For example, a functional test for a user registration feature might:
- Navigate to a registration page
- Fill in registration details
- Submit the form
- Verify that the system acknowledges successful registration
- Confirm the user can log in with the new credentials

### The Inner Loop: Unit Tests

Once the outer loop establishes what needs to be built, the inner loop guides how to build it through fine-grained unit tests. These tests:

1. **Target specific components**: Focus on individual classes, functions, or modules
2. **Drive implementation details**: Guide the precise implementation of each component
3. **Enable incremental progress**: Allow developers to make small, verifiable steps
4. **Support refactoring**: Provide confidence when improving code structure

For each piece of functionality needed to satisfy the outer loop test, developers write unit tests before implementing the code. This follows the traditional TDD cycle:

1. Write a failing unit test
2. Implement just enough code to make the test pass
3. Refactor the code while keeping tests green
4. Repeat until all required functionality is implemented

### Maintaining the Red-Green-Refactor Cycle at Both Levels

What makes Double Loop TDD powerful is the disciplined application of the red-green-refactor cycle at both levels:

1. **Red**: The outer loop test fails, indicating that the feature is not yet complete
2. **Green**: All inner loop tests pass, contributing incrementally to satisfying the outer test
3. **Refactor**: Code is improved without changing behavior, verified by both inner and outer tests
4. **Check outer loop**: Periodically run the outer loop test to track progress toward the overall goal

The outer test remains red until all necessary components have been developed through the inner loop. When the outer test finally passes, the feature is complete from the user's perspective.

### The Contract and Construction Plan

In this methodology:

- **The outer test becomes your "contract"**: It defines what the system must do to satisfy user requirements. This contract remains stable throughout development.
- **The inner loop becomes your "construction plan"**: It determines how the system will fulfill that contract, allowing for flexibility in implementation details.

This separation of concerns allows developers to focus on the right level of abstraction at each stage of development, ensuring that both user needs and technical quality are addressed.

## The Role of Spikes

While Outside-In TDD emphasizes a test-first approach, it also acknowledges the reality that developers sometimes face novel or unfamiliar challenges. In these situations, a time-boxed exploration called a "spike" can be a valuable preliminary step.

### Spikes as Exploratory Steps

A spike is a *valid exploratory step*, especially in uncertain domains where:

- The team lacks familiarity with a new technology or API
- System behavior is not well understood or documented
- Requirements are ambiguous or complex
- Technical approaches need validation before committing to a direction

During a spike, developers temporarily set aside test-first discipline to explore possibilities and constraints. This exploration typically involves writing experimental code without the normal test coverage requirements.

### Implementation on Throwaway Branches

Spikes are ideally performed on throwaway branches, emphasizing their temporary nature. This approach:

- Prevents untested code from entering the main codebase
- Emphasizes the exploratory nature of the work
- Signals to the team that this code is not production-ready
- Makes it easier to discard the code once learning is complete

### From Spike to Test-First Development

Once a spike yields sufficient insight, developers return to the test-first approach with:

- Clearer architectural vision
- Better understanding of constraints
- Validated technical approaches
- More precise requirements

The knowledge gained during the spike informs the subsequent TDD process, leading to more effective tests and implementation.

As Harry Percival aptly puts it: *"Spikes are temporary scaffolds that guide long-term structure."* Like scaffolding used in construction, spikes provide temporary support during exploration but are ultimately removed, leaving only the well-tested, production-ready code developed through disciplined TDD.

## Worked Example

To illustrate the Outside-In TDD approach in practice, let's walk through a generic example for a common feature requirement:

> "As a user, I want to upload a file and receive a processed result via email."

### Starting with the Outer Loop: The Functional Test

We begin by writing a functional test that captures the entire user journey:

```python
def test_user_can_upload_file_and_receive_processed_result_via_email():
    # Setup test data
    test_file = create_test_file()
    user_email = "user@example.com"
    
    # User navigates to the upload page
    browser.get("/upload")
    
    # User uploads a file
    file_input = browser.find_element_by_id("file-input")
    file_input.send_keys(test_file.path)
    
    # User provides their email
    email_input = browser.find_element_by_id("email-input")
    email_input.send_keys(user_email)
    
    # User submits the form
    submit_button = browser.find_element_by_id("submit-button")
    submit_button.click()
    
    # User sees confirmation message
    confirmation_message = browser.find_element_by_id("confirmation-message")
    assert "Your file has been uploaded and is being processed" in confirmation_message.text
    
    # Check that an email was sent to the user
    assert email_was_sent_to(user_email)
    
    # Verify the email contains the processed result
    email_content = get_email_content(user_email)
    assert "Processed result" in email_content
    assert "Download link" in email_content
```

At this point, running the test would fail immediately—no implementation exists to satisfy any part of this user journey.

### Inner Loop: Building Components Incrementally

With the outer test in place, we now begin the inner loop to build the required components incrementally.

#### 1. First, we need an upload page controller:

Unit test:
```python
def test_upload_page_renders_form():
    response = UploadController().get()
    assert response.status_code == 200
    assert "<form" in response.content
    assert 'id="file-input"' in response.content
    assert 'id="email-input"' in response.content
    assert 'id="submit-button"' in response.content
```

Implementation:
```python
class UploadController:
    def get(self):
        return Response(render_template("upload_form.html"), status=200)
```

#### 2. Next, we need to handle file upload submissions:

Unit test:
```python
def test_upload_controller_processes_submission():
    file_data = MockFile("test.txt", b"test content")
    email = "user@example.com"
    
    controller = UploadController()
    response = controller.post(file=file_data, email=email)
    
    assert response.status_code == 200
    assert "confirmation-message" in response.content
    assert FileProcessingQueue.has_job_for(email=email)
```

Implementation:
```python
class UploadController:
    # ...existing methods...
    
    def post(self, file, email):
        # Save file to temporary storage
        file_path = save_to_temp_storage(file)
        
        # Queue file for processing
        FileProcessingQueue.add_job(file_path=file_path, email=email)
        
        # Return confirmation page
        return Response(render_template("confirmation.html"), status=200)
```

#### 3. Then, we need a file processing service:

Unit test:
```python
def test_file_processor_handles_job():
    # Setup
    file_path = "/tmp/test_file.txt"
    email = "user@example.com"
    job = FileProcessingJob(file_path=file_path, email=email)
    
    # Execute
    processor = FileProcessor()
    result = processor.process_job(job)
    
    # Verify
    assert result.success
    assert result.processed_data is not None
    assert EmailService.was_called_with(
        recipient=email,
        subject="Your file has been processed",
        content=Contains("Processed result")
    )
```

Implementation:
```python
class FileProcessor:
    def process_job(self, job):
        # Process the file
        processed_data = self._apply_processing(job.file_path)
        
        # Store result
        result_id = ResultStorage.save(processed_data)
        
        # Generate download link
        download_link = self._generate_download_link(result_id)
        
        # Send email
        EmailService.send(
            recipient=job.email,
            subject="Your file has been processed",
            content=self._create_email_content(download_link)
        )
        
        return ProcessingResult(success=True, processed_data=processed_data)
    
    def _apply_processing(self, file_path):
        # Actual file processing logic
        # ...
```

#### 4. Finally, we need an email service:

Unit test:
```python
def test_email_service_sends_email():
    # Setup
    recipient = "user@example.com"
    subject = "Test Subject"
    content = "Test Content"
    
    # Execute
    result = EmailService.send(
        recipient=recipient,
        subject=subject,
        content=content
    )
    
    # Verify
    assert result.success
    assert EmailProvider.was_called_with(
        to=recipient,
        subject=subject,
        body=content
    )
```

Implementation:
```python
class EmailService:
    @staticmethod
    def send(recipient, subject, content):
        # Call email provider API
        response = EmailProvider.send_email(
            to=recipient,
            subject=subject,
            body=content
        )
        
        # Log the email send attempt
        EmailLog.create(
            recipient=recipient,
            subject=subject,
            success=response.success
        )
        
        return EmailResult(success=response.success)
```

### Final Pass State and Clean Architecture

Once all these components are implemented and their unit tests pass, we run the outer functional test again. If all the components work together correctly, the functional test should now pass, indicating that the feature is complete from the user's perspective.

The resulting architecture is clean and well-structured:
- **Controllers** handle HTTP requests and responses
- **Services** encapsulate business logic
- **Queues** manage asynchronous processing
- **External integrations** are cleanly separated

Each component has a single responsibility and clear interfaces, making the system easier to understand, maintain, and extend.

This architecture emerged naturally from the Outside-In TDD process rather than being imposed upfront. By starting with the user journey and working inward, we developed exactly the components needed to fulfill the requirement, no more and no less.

## Benefits

Outside-In TDD offers numerous advantages for teams developing complex software systems:

### Alignment with User Needs

By starting with tests that represent user journeys, Outside-In TDD ensures development efforts remain focused on delivering actual value rather than implementing technically impressive solutions that miss the mark for users. The outer loop tests act as a constant reminder of the user's perspective, helping to prevent feature creep and unnecessary complexity.

Even when technical challenges arise during implementation, having clear functional tests helps developers make decisions that prioritize user experience over technical convenience. This alignment is particularly valuable in agile environments where requirements evolve based on user feedback.

### Reduction of Wasted Effort

Traditional approaches can lead to wasted effort in several ways:
- Building components that aren't needed
- Over-engineering solutions to anticipated but unrealized requirements
- Spending time on optimizations that don't matter to users

Outside-In TDD mitigates these risks by driving development directly from user requirements. Each component exists to fulfill a specific user need, and its interfaces are designed based on actual usage patterns rather than speculative abstractions.

As Harry Percival notes, "Outside-In TDD helps us build the simplest thing that could possibly work." This focus on simplicity leads to more maintainable and adaptable codebases.

### Encouragement of Modularity

The methodology naturally encourages the development of modular, testable, and maintainable code. Because the inner loop requires components to be unit-testable, developers must:
- Define clear interfaces
- Maintain separation of concerns
- Limit coupling between components
- Design for testability

These practices lead to systems that are not only well-tested but also easier to understand, maintain, and extend. The modular architecture that emerges from Double Loop TDD typically exhibits good cohesion (components have clear, focused responsibilities) and loose coupling (components interact through well-defined interfaces).

### Improved Team Communication

Outside-In TDD provides significant benefits for cross-functional teams:

- **Developers and Product Managers** share a common language of user journeys and acceptance criteria
- **QA and Development** have a shared understanding of expected behavior through executable functional tests
- **New team members** can quickly understand the system's behavior by reading functional tests
- **Cross-functional collaboration** is enhanced when all roles can contribute to defining the outer loop tests

The functional tests serve as living documentation of the system's behavior, reducing misunderstandings and ensuring everyone works toward the same goals.

## Tradeoffs

While Outside-In TDD offers substantial benefits, it also involves certain tradeoffs and challenges that teams should consider:

### Requires Strong Test Design Skills

Double Loop TDD demands strong test design skills at both the functional and unit levels:

- **Functional tests** must be comprehensive yet focused, capturing essential user journeys without becoming brittle or over-specified
- **Unit tests** require careful boundary definitions and appropriate use of test doubles
- **Both levels** must strike a balance between coverage and maintainability

Teams new to the approach may initially struggle to find this balance. Functional tests that are too detailed become brittle, while those that are too general may miss critical edge cases. Similarly, unit tests must verify behavior without being tightly coupled to implementation details.

Developing these skills requires practice, mentorship, and ongoing refinement of testing practices.

### Initial Brittleness of Outer Tests

When first implementing a feature, outer loop tests can feel brittle or overly abstract:

- Tests may initially fail in unpredictable ways as the implementation evolves
- Functional tests interact with the system at its boundaries, making them sensitive to UI changes, API restructuring, or other interface modifications
- End-to-end tests typically run more slowly than unit tests, providing longer feedback cycles

Teams can mitigate these challenges by:
- Focusing functional tests on critical user journeys rather than edge cases
- Using stable selectors or identifiers for UI elements
- Separating interface details from business logic assertions
- Investing in test infrastructure to improve reliability and performance

As the system matures, these tests become valuable guardians of expected behavior, but the initial investment can be substantial.

### Requires Disciplined Approach

Outside-In TDD demands disciplined adherence to its principles to yield the full benefits:

- **Temptation to skip ahead**: Developers may be tempted to jump directly to implementation without writing tests first
- **Pressure to cut corners**: Project deadlines or complexity may tempt teams to abandon the methodology in challenging situations
- **Consistency across team members**: All developers must follow the approach for it to be effective

This discipline is particularly challenging when working with legacy systems, complex domains, or under tight deadlines. Teams must commit to the methodology and support each other in maintaining the discipline it requires.

### Learning Curve

For teams accustomed to other development approaches, Outside-In TDD involves a significant learning curve:

- Developers must learn to think from the user's perspective before diving into implementation
- The double loop concept requires managing two levels of tests simultaneously
- Team members must develop skills in both functional and unit testing
- New testing tools and frameworks may need to be adopted

This learning curve can temporarily reduce productivity as the team adapts to the new approach. However, most teams find that the investment pays dividends through improved quality and maintainability.

## Comparison with Other Methodologies

To better understand Outside-In TDD, it's valuable to compare it with other prevalent software development methodologies:

### Traditional Bottom-Up TDD

Traditional TDD, as popularized by Kent Beck, typically follows a bottom-up approach:

- **Starting point**: Individual components or functions
- **Focus**: Building reliable components that can be assembled into a whole
- **Testing scope**: Primarily unit tests with integration tests added later

**Key differences from Outside-In TDD**:
1. Bottom-up TDD lacks the explicit outer loop that ensures user requirements are met
2. Integration of components is usually addressed later in the development process
3. User perspective may be less prominently represented in the testing strategy

While traditional TDD excels at creating well-tested components, it can sometimes lead to what Percival calls the "test-driven design trap"—building perfectly tested components that don't quite meet user needs when integrated.

### Behavior-Driven Development (BDD)

BDD, popularized by Dan North, has several similarities to Outside-In TDD:

- **Starting point**: User behaviors and business requirements
- **Focus**: Delivering features that provide business value
- **Testing scope**: Multi-level, with acceptance tests driving development

**Key differences from Outside-In TDD**:
1. BDD places greater emphasis on collaboration and shared language between technical and non-technical stakeholders
2. BDD often uses specialized formats like Gherkin for expressing scenarios
3. BDD doesn't necessarily prescribe the inner TDD loop, though many practitioners combine the approaches

Outside-In TDD can be seen as complementary to BDD, with BDD providing the collaborative framework for defining requirements and Outside-In TDD offering the technical practices for implementing them.

### Prototype-First or Spike-Driven Development

Some teams prefer to begin with prototypes or spikes before moving to test-driven approaches:

- **Starting point**: Exploratory coding to understand the problem space
- **Focus**: Learning and discovery before commitment
- **Testing scope**: Tests often added after prototype validation

**Key differences from Outside-In TDD**:
1. Prototype-first approaches delay testing until after initial exploration
2. Code quality may be sacrificed during the prototype phase
3. Transition from prototype to production code can be challenging

Outside-In TDD acknowledges the value of spikes but keeps them as temporary, disposable explorations rather than foundations for the final implementation.

### Exploratory Coding Without Regression Safety Nets

Some developers prefer a more freestyle approach to coding:

- **Starting point**: Direct implementation based on requirements
- **Focus**: Rapid development and immediate feedback
- **Testing scope**: Manual testing or tests added after implementation

**Key differences from Outside-In TDD**:
1. Exploratory coding lacks the safety nets that TDD provides
2. Regression issues may emerge as the system evolves
3. Code design may be more influenced by implementation convenience than user needs

Outside-In TDD offers more structure and discipline, trading some initial development speed for longer-term maintainability and alignment with user needs.

### Comparative Strengths

Each methodology has contexts where it shines:

- **Traditional Bottom-Up TDD**: Excellent for libraries, frameworks, and components with well-defined interfaces
- **BDD**: Strong for customer-facing applications where business stakeholder collaboration is critical
- **Prototype-First**: Valuable for novel domains or when requirements are highly uncertain
- **Exploratory Coding**: Can be efficient for small, short-lived projects or proof-of-concepts

**Outside-In TDD's sweet spot**: Systems with complex user interactions spanning multiple layers of architecture, particularly web applications, service-oriented systems, and applications integrating multiple technologies.

## Conclusion

Outside-In Test-Driven Development represents a powerful evolution of test-driven practices that is particularly well-suited to the challenges of modern software development. By working from user requirements inward to implementation details, it ensures that development efforts remain focused on delivering genuine value.

### Building the Right Thing and Building the Thing Right

The double loop approach addresses two fundamental challenges in software development:
- **Building the right thing**: The outer loop ensures the system meets actual user needs
- **Building the thing right**: The inner loop ensures the implementation is correct and maintainable

This dual focus helps teams avoid the common pitfall of creating technically excellent solutions that miss the mark for users. It also helps prevent the opposite problem of building user-friendly systems that cannot be maintained or extended over time.

### The Productive Tension Between Design Intent and Implementation

Outside-In TDD creates a productive tension between high-level design intent and low-level implementation details. This tension:
- Prevents premature optimization or over-engineering
- Encourages appropriate abstractions based on actual usage patterns
- Maintains focus on both external quality (user experience) and internal quality (code structure)

As development progresses, this tension naturally leads to systems that balance user needs with technical considerations.

### Recommendation for Modern Systems

Outside-In TDD is particularly recommended for teams building:
- **Web applications** with complex user interactions
- **Systems integrating multiple services** or APIs
- **Applications with rich user interfaces**
- **Software where user experience is critical to success**
- **Systems that must evolve over time in response to changing requirements**

For these contexts, the methodology provides a structured approach to managing complexity while maintaining focus on user needs.

As software systems continue to grow in complexity and user expectations continue to rise, approaches like Outside-In TDD become increasingly valuable. By starting with what the user wants to do and working inward to implementation details, teams can build systems that are both user-focused and technically sound.

## Appendix

### Sample Outer-Loop Test

The following example shows a typical outer-loop test for a web application, using Selenium for browser automation:

```python
def test_user_can_create_and_retrieve_a_todo_item():
    # User visits the home page
    browser.get("http://localhost:8000")
    
    # User sees the page title
    assert "Todo App" in browser.title
    
    # User enters a new todo item
    input_box = browser.find_element_by_id("new-todo")
    input_box.send_keys("Buy peacock feathers")
    input_box.send_keys(Keys.ENTER)
    
    # The page updates and shows the item in the todo list
    todo_list = browser.find_element_by_id("todo-list")
    todo_items = todo_list.find_elements_by_tag_name("li")
    assert "Buy peacock feathers" in [item.text for item in todo_items]
    
    # User adds another item
    input_box = browser.find_element_by_id("new-todo")
    input_box.send_keys("Use peacock feathers to make a fly")
    input_box.send_keys(Keys.ENTER)
    
    # The page updates and shows both items
    todo_list = browser.find_element_by_id("todo-list")
    todo_items = todo_list.find_elements_by_tag_name("li")
    assert "Buy peacock feathers" in [item.text for item in todo_items]
    assert "Use peacock feathers to make a fly" in [item.text for item in todo_items]
    
    # User closes the browser and reopens it
    browser.quit()
    browser = webdriver.Firefox()
    browser.get("http://localhost:8000")
    
    # The todo items are still there, showing persistence
    todo_list = browser.find_element_by_id("todo-list")
    todo_items = todo_list.find_elements_by_tag_name("li")
    assert "Buy peacock feathers" in [item.text for item in todo_items]
    assert "Use peacock feathers to make a fly" in [item.text for item in todo_items]
```

### Sample Inner-Loop Tests

The following examples show typical inner-loop tests that might be written to implement functionality needed for the outer-loop test:

**Model Test:**
```python
def test_todo_item_can_be_created_and_retrieved():
    # Create a todo item
    TodoItem.create(text="Buy peacock feathers")
    
    # Retrieve all items
    items = TodoItem.get_all()
    
    # Verify the item was saved
    assert len(items) == 1
    assert items[0].text == "Buy peacock feathers"
```

**View Test:**
```python
def test_home_page_renders_todo_list():
    # Create some todo items
    TodoItem.create(text="Item 1")
    TodoItem.create(text="Item 2")
    
    # Render the home page
    response = HomeView().get()
    
    # Verify that the items are in the response
    assert response.status_code == 200
    assert "Item 1" in response.content
    assert "Item 2" in response.content
```

**Controller Test:**
```python
def test_create_todo_controller_adds_new_item():
    # Initial state: no items
    assert len(TodoItem.get_all()) == 0
    
    # Send a request to create a new item
    controller = CreateTodoController()
    response = controller.post(text="Buy milk")
    
    # Verify the response
    assert response.status_code == 302  # Redirect
    
    # Verify that an item was created
    items = TodoItem.get_all()
    assert len(items) == 1
    assert items[0].text == "Buy milk"
```

### Suggested Test Structure for Monorepos

For projects organized as monorepos with multiple services or components, consider the following test structure:

```
/
├── e2e/                           # End-to-end tests (outer loop)
│   ├── journeys/                  # Tests organized by user journey
│   │   ├── user_registration.spec.js
│   │   ├── todo_management.spec.js
│   │   └── ...
│   ├── fixtures/                  # Test data for e2e tests
│   └── helpers/                   # Helper functions for e2e tests
│
├── services/                      # Individual services/components
│   ├── auth-service/              # Authentication service
│   │   ├── src/                   # Source code
│   │   └── tests/                 # Unit tests (inner loop)
│   │       ├── unit/              # Pure unit tests
│   │       └── integration/       # Tests that cross module boundaries
│   │
│   ├── todo-service/              # Todo management service
│   │   ├── src/
│   │   └── tests/
│   │       ├── unit/
│   │       └── integration/
│   │
│   └── ...
│
├── libs/                          # Shared libraries
│   ├── common-utils/
│   │   ├── src/
│   │   └── tests/                 # Unit tests for shared code
│   └── ...
```

This structure separates the outer loop (e2e tests) from the inner loop (unit and integration tests), while organizing tests logically alongside the code they verify.

### Tips for Managing Spike Branches

When using spikes as part of Outside-In TDD, follow these best practices:

1. **Clear naming convention**: Use a prefix like `spike/` for spike branches to distinguish them from feature branches
2. **Document the purpose**: Include a clear description of the spike's goals in the branch name or commit message
3. **Time-box exploration**: Set a specific time limit for the spike to prevent indefinite exploration
4. **Extract learning, not code**: Focus on extracting knowledge from the spike, not reusing its code
5. **Throwaway mindset**: Approach spike code as disposable, emphasizing learning over craftsmanship
6. **Document findings**: Summarize what was learned before discarding the spike
7. **Return to TDD**: After completing the spike, start fresh with test-first development

Example workflow:
```bash
# Create a spike branch
git checkout -b spike/explore-file-processing-api

# ... Exploratory coding ...

# Document findings in a markdown file
echo "# File Processing API Spike Findings" > spike-findings.md
# ... Add details about what was learned ...

# Commit the findings
git add spike-findings.md
git commit -m "Document findings from file processing API spike"

# Extract findings to the main branch
git checkout main
git checkout spike/explore-file-processing-api -- spike-findings.md
git commit -m "Add findings from file processing API spike"

# Begin TDD implementation
git checkout -b feature/file-processing
# ... Begin with outer loop test ...
```

By following these practices, teams can benefit from exploratory coding while maintaining the discipline of Outside-In TDD for production code.
