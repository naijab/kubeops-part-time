# kubeops-part-time - Question

## Normal

1. Describe Git branching strategies (Git-flow, single branch, feature branch etc.) which you have used and what purpose does it serves.

> ANS: ใช้ git-flow เป็นการแบ่ง branch เพื่อใช้ประโยชน์ รวมถึงแยกโค้ดที่ทดสอบแล้ว หรือกำลังพัฒนาออกจากกัน
> **master** เป็น branch ของโค้ดที่เสร็จแล้ว ทดสอบเรียบร้อยแล้ว หรือเรียกว่า production รวมถึงจะติด tag version เพื่อแยก version ใหม่เก่าออกจากกัน และให้ง่ายต่อการ deploy หรือ rollback
> **develop** เป็น branch หลักของโปรเจค เพราะหากมี feature ใหม่ๆ จะต้อง merge มารวมกันที่นี่ก่อน รวมถึงเพื่อใช้ทดสอบก่อนจริงก่อนขึ้น production อีกทั้ง develop จะแตก branch มาจาก master เพื่อที่จะได้ pull request กลับเข้า master เมื่อพร้อมจะ deploy production
> **feature** เป็น branch ย่อยที่แตกจาก develop เพื่อพัฒนา หรือแก้ไขฟังก์ชันต่างๆ ที่ต้องแยกเพื่อให้คนในทีม สามารถพัฒนาไปพร้อมๆ กันได้ และเมื่อเสร็จจะ merge กลับเข้า develop
> **hotfix** เป็น branch ย่อยที่แตกมาจาก master เพื่อแก้ไข bug ที่อาจเกิดขึ้นได้ หากเร่งด่วนจะ merge กลับเข้า master แต่ถ้าไม่ด่วนมาก จะ merge เข้า develop เพื่อรอทดสอบต่อไป

2. How do you revert a commit that has already been pushed and made public?

> ANS: สามารถใช้คำสั่ง `git reset` หรือ `git revert` โดยทั้งสองตัวมีข้อแตกต่างกันคือ
> **git reset** จะทำการ reset history หรือลบ commit นั้น ๆ หากเลือก mode hard ซึ่งต้องมั่นใจว่าที่ reset นั้นถูกต้อง เพราะหาก git force push แล้วคนอื่น pull ลงมาจะย้อนกลับไม่่ได้
> **git revert** จะทำการยกเลิกการแก้ไขของ commit นั้น โดยจะ commit ใหม่ต่ออีกที แนะนำวิธีนี้เพราะปลอดภัย เพราะจะยังคงมี history เหลืออยู่

3. How do you normally solve conflicts in a feature branch before merge?

> ANS: ใช้คำสั่ง **git rebase** ก่อนเพื่อป้องกันการ conflict โดยเมื่อใช้ git rebase จะเป็นการนำ history ของเส้นที่จะ rebase มาเป็นฐานของ branch ปัจจุบัน เมื่อเกิด conflict ก็จะสามารถแกัไขได้ ก่อน pull request หรือ merge ในอนาคต

4. “200 OK” what does it mean and show use case this HTTP Status?

> ANS: เป็น http status ที่ใช้สื่อสารกับ client ว่าการทำงานนั้นเสร็จสมบูรณ์แล้ว เช่น เปิดหน้าเว็บไชต์ถ้าเปิดได้ปกติ ก็จะได้ 200 OK รวมถึงการดาวน์โหลดไฟล์ต่าง ๆ ที่ใช้ในเว็บนั้น ๆ 
> - อีกทั้งยังใช้ในการตรวจสอบการติดต่อระหว่าง server สองตัว ว่าปกติดีหรือไม่

5. “201 Created” what does it mean and show use case this HTTP Status?

> ANS: เป็น status ที่ใช้ในการแจ้ง client ว่าการสร้างนั้นๆ เสร็จเรียบร้อยดี เช่น การเพิ่มรายการสินค้า ทั้งนี้อาจจะต้องตกลงในทีมให้เรียบร้อย เพราะบางคนจะตีความใช้ 200 บางคน 201 ดังนั้น ต้องชัดเจน

6. “301 Moved Permanently” what does it mean and show use case this HTTP Status?

> ANS: เป็น status ที่บอก client ว่ามีการ redirect url นึงไปอีกที่ เช่น เข้า url ผ่าน A แต่มีการ redirect ต่อไปยัง B 

7. “400 Bad Request” what does it mean and how to identify the problem?

> ANS: เป็น status บอก client ว่าการส่ง request มีปัญหา เช่น การข้อมูลที่ส่งไปยัง server นั้นมีข้อมูลไม่ถูกต้อง อาจเพราะใช้ email ซ้ำ หรือ รูปแบบวันที่ผิดจากที่ server นั้นต้องการ
> ควรจะต้องแจ้ง user ที่กำลังใช้งานว่า ผิดที่ข้อมูลส่วนไหน เช่น เบอร์โทร ไม่ครบ 10 ตัว

8. “401 Unauthorized” what does it mean and how to identify the problem?

> ANS: เป็น status บอก client ว่าไม่ได้ยืนยันตัวตน ทำให้เข้าถึงไม่ได้ ดังนั้นมักจะ redirect ให้ login ก่อน
> การระบุปัญหา จะต้องตรวจดูว่า server มีการ auth อย่างไร มีการใช้ token หรือ cookie หรือไม่ ถ้าใช้ ดูว่ามีครบ หรือส่งไปกับ header หรือไม่

9. “403 Forbidden” what does it mean and how to identify the problem?

> ANS: เป็น status บอก client ว่าไม่มีสิทธิ์ในการเข้าถึง เช่น user ไม่มีสิทธิ์ในหน้า admin หรือระดับสิทธิ์ไม่ถึงตามที่ server กำหนด 
> การระบุปัญหา จะต้องตรวจสอบว่า user ที่ใช้งานมีสิทธิ์ หรือระดับ user กับ api นั้น ๆ หรือไม่

10.   “404 Not Found” what does it mean and how to identify the problem?

> ANS: เป็น status บอก client ว่าหน้าที่กำลังเข้าถึงนั้นไม่มีอยู่ หรือไม่พบ รวมถึง server แจ้งว่าไม่พบ เช่น user id หรือ product id ที่เรียกอยู่
> การระบุปัญหา จะต้องลองค้นหาไฟล์ที่อยู่บน server ว่ามีอยู่จริงหรือไม่ หรือชื่อเปลี่ยนไป path ถึงต้องไหม

11.  “500 Internal Server Error” what does it mean and how to identify the problem?

> ANS: เป็น status ที่แจ้ง client ว่ามีปัญหาบางอย่างที่ server ทำให้ไม่สามารถให้บริการได้ อาจจะเกิดจากการล่มของฐานข้อมูล ทำให้ server ไม่สามารถตอบสนอง client ได้
> การระบุปัญหา จะต้องดูจาก log ที่มีการเก็บไว้ ว่าเกิดที่ service ใดๆ ช่วงเวลาไหน เพื่อที่จะได้แก้ไขได้ถูกต้อง

12. “502 Bad Gateway” what does it mean and how to identify the problem?

> ANS: เป็น status ที่แจ้ง client ว่ามีปัญหาจากการติดต่อ server เช่น proxy ก่อนเข้าถึง backend มีปัญหา
> การระบุปัญหา ดู log ประกอบ รวมถึง check status ของ proxy หรือบริการที่อาจจะมีปัญหา ว่ายังทำงานได้ครบถ้วน หรือไม่

13. “503 Service Unavailable” what does it mean and how to identify the problem?

> ANS: เป็น status ที่แจ้ง client ว่ามี service มีปัญหา เช่น อาจจะรับ request ไม่ไหว ดังนั้น หากมี 503 เกิดขึ้นแสดงว่า อาจจะต้องดูว่า service หรือเครื่อง server เรามี spec ที่เพียงพอแล้วหรือยัง หรือน้อยเกินไป ในการให้บริการ
> การระบุปัญหา ดู log ประกอบ รวมถึงดูว่าช่วงเวลาที่มีปัญหา สัมพันธ์กันอย่างไรกับปริมาณ request อาจจะลองเพิ่ม spec หรือ container แล้วลองยิ่ง request เพื่อทดสอบการรองรับ ว่าเพียงพอ หรือเผื่อไว้มากพอ

14. “504 Gateway Timeout” what does it mean and how to identify the problem?

> ANS: เป็น status ที่แจ้งว่า service มีปัญหา เพราะเนื่องจาก ระยะเวลาที่ใช้ทำการนานไป และอาจส่งผลต่อ user คนอื่น จึงต้องตัด connection ที่รอนาน เพื่อลดปัญหาทั้งระบบ
> การระบุปัญหา จะต้องทดสอบการยิง request จาก vpn หลายๆ ที่ เพื่อทดสอบว่า กลุ่มลูกค้าที่ใช้ มีปัญหามากน้อย แค่ไหน อาจต้องเพิ่ม sever, cdn ที่ประเทศนั้นๆ

15. What are Linux network tools do you use for troubleshooting network problems as well as usage scenarios for each tool?

> ANS: 
> **ping** เป็น tool ในการทดสอบการติดต่อระหว่าง server ว่าสามารถติดต่อได้หรือไม่ มักจะเป็น tools ตัวแรกในการตรวจสอบปัญหา รวมถึงดูระยะเวลาการตอบสนอง ว่านานเกินไปหรือไม่ 
> ตัวที่สอง หาทดสอบเชื่อมต่อด้วย ping แล้ว ยังปกติ ต้องหาว่า service นั้นๆ ทำงานอยู่หรือไม่ โดยใช้ **netstat** ใช้ในการแสดง port ต่างๆ ที่ถูกใช้ในเครื่อง สุดท้ายตรวจสอบว่า domain ที่ผูกไปยิงไป ip ถูกต้องไหม
> **nslookup** ใช้ในการหา ip ที่ผูกกับ domain name เพื่อให้รู้ว่า domain name นั้นผูกกับ ip ถูกต้องหรือไม่
---

## Intermediate

1. Assume we have an application that is designed as below. Our application stopped responding due to an extremely high number of clients in some circumstances.
We have tried scaling a number of API Gateway and Service A nodes but it didn’t help. What are the possible problems that lies in our system in which components and how to fix them?

> ANS:

2. How do you keep the docker image smallest as possible?

> ANS:

3. What is the difference between overlay, bridge, host network in Docker? When to use each of them?

> ANS:

4. How does the Kubernetes service talk to each other in the same cluster?

> ANS:

5. What’s different between L2, L4, and L7 Load balancers? When to use it?

> ANS:

---

## Professional

1. Assume that you are using a private cloud for your infrastructure. How do you manage logs, metrics, and alerts for your infrastructure and applications? Which tools do you use and why?

> ANS:

2. How do you secure the following?
   - application
   - infrastructure
   - data

> ANS:

3. Base on your experience, how do you reduce your service downtime as much as possible during
   - software upgrade
   - database migration
   - incident

> ANS:

4. Configuration management
  a. Which Among Puppet, chef, Ansible, or another is the best Configuration management tool?
  b. Why?
  c. Do you still need to use it if you already have docker-swarm or Kubernetes?

> ANS:

5. How do you design your Kubernetes cluster? what DNS, CNI, ingression is being used? Why?

> ANS:

6. How do you measure service quality to give the best experience to your customer? (SLO, SLA)

> ANS:

---

## Hands-On

1. Please build and deploy your application of choice using Kubernetes. It should support multiple environments (e.g. beta, prod).
The solution must include the following

- Architecture Diagram
- Dockerfile or Buildpacks supports the following languages:
  - .Net Core
  - Go
  - Java
  - Node
  - Rust

- CI/CD pipeline file:
  You can choose your CI/CD tool:
  - Gitlab CI
  - Github Actions
  - CircleCI
  - Jenkins
  - Travis CI
  - GoCD
  - BitBucket
  - Azure DevOps
- Kubernetes Manifests
- Kubernetes Ingress Configuration

Please update README for steps how to test, build, deploy and run the application using Kubernetes

> ANS:

---

## Algorithms

Choose your favorite language (Rust would be advantages) to solve the following challenges:

1. FizzBuzz...But: You may heard the FizzBuzz task. Here we have the same rule. You will write a function fizzBuzz that receive a single parameter it will return the value base on these rule.

- If the input is divisible by 3, return 'Fizz'
- If the input is divisible by 5, return 'Buzz'
- If the input is divisible by both 3 and 5, return 'FizzBuzz'

BUT we're not allow you to use if/else statement. If there is any if or else word in your code. I will consider this question with 0 scores :(

### Example

```
> fizzBuzz(21)
Fizz

> fizzBuzz(25)
Buzz

> fizzBuzz(45)
FizzBuzz
```

2. Bob has a server farm crunching numbers. He has nodes servers in his farm. His company has a lot of work to do. The work comes as a number workload which indicates how many jobs there are. Bob wants his servers to get an equal number of jobs each. If that is impossible, he wants the first servers to receive more jobs. He also wants the jobs sorted, so that the first server receives the first jobs. The way this works, Bob wants an array indicating which jobs are going to which servers.
Can you help him distribute all this work as evenly as possible onto his servers?

### Example

Bob has 2 servers and 4 jobs. The first server should receive job 0 and 1 while the second should receive 2 and 3.

```
distribute(2, 4) # => [[0, 1], [2, 3]]
```

On a different occasion Bob has 3 servers and 3 jobs. Each should get just one.

```
distribute(3, 3) # => [[0], [1], [2]]
```

A couple of days go by and Bob sees a spike in jobs. Now there are 10, but he hasn't got more than 4 servers available. He boots all of them. This time the first and second should get a job more than the third and fourth.

```
distribute(4, 10) # => [[0, 1, 2], [3, 4, 5], [6, 7], [8, 9]]
```

3. It's tricky keeping track of who is owed what when spending money in a group. Write a function to balance the books.

- The function should take one parameter: a dict with two or more name-value pairs which represent the members of the group and the amount spent by each.
- The function should return a dict with the same names, showing how much money the members should pay or receive.

Further points:

- The values should be positive numbers if the person should receive money from the group, negative numbers if they owe money to the group.
- If value is a decimal, round to two decimal places.

### Example

3 friends go out together: A spends $20, B spends $15, and C spends $10. The function should return an object/dict showing that A should receive $5, B should receive $0, and
C should pay $5.

```
var group = {
    A: 20, 
    B: 15, 
    C: 10
}
 
splitTheBill(group) // returns {A: 5, B: 0, C: -5}

```

4. Fibonacci
The Fibonacci numbers are the numbers in the following integer sequence.

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, …
```

In mathematical terms, the sequence Fn of Fibonacci numbers is defined by the recurrence relation

Fn = Fn-1 + Fn-2

with seed values

F0 = 0 and F1 = 1.

Given a number n, print n-th Fibonacci Number.

5. Palindrome
Given a string, write a function to check if it is palindrome or not.

In case: even you can reverse the string

In case: odd you should use the middle to split the word and then reverse the string

A string is said to be palindrome if the reverse of the string is the same as the string. For example, “abba” is Palindrome, but “abbc” is not Palindrome.

Anna, civic, kayak, level, madam, mom, noon, racecar, radar, redder, refer, repaper, rotator, 12321, 15651
